#![cfg(windows)]

use crate::infra::winapi::ddc::sys::api::{
    CapabilitiesInfo,
    HMONITOR,
    PhysicalMonitorInfo,
    VcpFeature,
};
use crate::infra::winapi::ddc::sys::caps_parse::parse_capabilities;
use crate::infra::winapi::ddc::sys::error::{DdcError, Result};

use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};

use windows_sys::core::BOOL;
use windows_sys::Win32::Devices::Display::{
    DestroyPhysicalMonitor,
    GetCapabilitiesStringLength,
    GetNumberOfPhysicalMonitorsFromHMONITOR,
    GetVCPFeatureAndVCPFeatureReply,
    SetVCPFeature,
    MC_VCP_CODE_TYPE,
};
use windows_sys::Win32::Foundation::{HANDLE, LPARAM, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    EnumDisplayMonitors,
    GetMonitorInfoW,
    HDC,
    MONITORINFOEXW,
};

const PHYSICAL_MONITOR_DESCRIPTION_SIZE: usize = 128;

#[link(name = "Dxva2")]
unsafe extern "system" {
    fn GetPhysicalMonitorsFromHMONITOR(
        hmonitor: HMONITOR,
        dwphysicalmonitorarraysize: u32,
        pphysicalmonitorarray: *mut RawPhysicalMonitor,
    ) -> BOOL;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RawPhysicalMonitor {
    hphysicalmonitor: HANDLE,
    description: [u16; PHYSICAL_MONITOR_DESCRIPTION_SIZE],
}

struct MonitorGuard {
    physical_monitors: Vec<RawPhysicalMonitor>,
}

impl MonitorGuard {
    fn new(physical_monitors: Vec<RawPhysicalMonitor>) -> Self {
        Self { physical_monitors }
    }

    fn as_slice(&self) -> &[RawPhysicalMonitor] {
        &self.physical_monitors
    }
}

impl Drop for MonitorGuard {
    fn drop(&mut self) {
        for pm in &self.physical_monitors {
            unsafe {
                let _ = DestroyPhysicalMonitor(pm.hphysicalmonitor);
            }
        }
    }
}

fn last_os_error() -> std::io::Error {
    std::io::Error::last_os_error()
}

fn wide_nul_terminated_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    OsString::from_wide(&buf[..len])
        .to_string_lossy()
        .into_owned()
}

pub(super) fn get_device_name(hmonitor: HMONITOR) -> Option<String> {
    unsafe {
        let mut info: MONITORINFOEXW = zeroed();
        info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;

        let ok = GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _);
        if ok == 0 {
            return None;
        }

        let s = wide_nul_terminated_to_string(&info.szDevice);
        if s.is_empty() { None } else { Some(s) }
    }
}

fn get_friendly_name(pm: &RawPhysicalMonitor) -> Option<String> {
    let s = wide_nul_terminated_to_string(&pm.description);
    if s.is_empty() { None } else { Some(s) }
}

pub(super) fn enum_hmonitors() -> Result<Vec<HMONITOR>> {
    unsafe extern "system" fn enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &mut *(lparam as *mut Vec<HMONITOR>);
        monitors.push(hmonitor);
        1
    }

    let mut monitors = Vec::<HMONITOR>::new();

    unsafe {
        let ok = EnumDisplayMonitors(
            null_mut(),
            null(),
            Some(enum_proc),
            (&mut monitors as *mut Vec<HMONITOR>) as LPARAM,
        );

        if ok == 0 {
            return Err(DdcError::EnumDisplayFailed(last_os_error()));
        }
    }

    Ok(monitors)
}

fn get_physical_monitors(hmonitor: HMONITOR) -> Result<Vec<RawPhysicalMonitor>> {
    unsafe {
        let mut count: u32 = 0;
        if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut count) == 0 {
            return Err(DdcError::GetPhysicalMonitorsFailed(last_os_error()));
        }

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut arr: Vec<RawPhysicalMonitor> = vec![zeroed(); count as usize];
        if GetPhysicalMonitorsFromHMONITOR(hmonitor, count, arr.as_mut_ptr()) == 0 {
            return Err(DdcError::GetPhysicalMonitorsFailed(last_os_error()));
        }

        Ok(arr)
    }
}

pub(super) fn list_physical_monitor_infos() -> Result<Vec<PhysicalMonitorInfo>> {
    let mut out = Vec::new();

    for hmon in enum_hmonitors()? {
        let device_name = get_device_name(hmon).unwrap_or_else(|| "<unknown>".to_string());
        let pms = get_physical_monitors(hmon)?;
        let guard = MonitorGuard::new(pms);

        for (i, pm) in guard.as_slice().iter().enumerate() {
            out.push(PhysicalMonitorInfo {
                hmonitor: hmon,
                pm_index: i,
                device_name: device_name.clone(),
                friendly_name: get_friendly_name(pm),
            });
        }
    }

    Ok(out)
}

fn get_capabilities_string(pm_handle: HANDLE) -> Result<String> {
    unsafe {
        let mut length: u32 = 0;
        if GetCapabilitiesStringLength(pm_handle, &mut length) == 0 {
            return Err(DdcError::GetCapabilitiesFailed(last_os_error()));
        }

        if length == 0 {
            return Err(DdcError::GetCapabilitiesFailed(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Capabilities length is zero",
            )));
        }

        let mut buf = vec![0u8; length as usize];
        let ok = windows_sys::Win32::Devices::Display::CapabilitiesRequestAndCapabilitiesReply(
            pm_handle,
            buf.as_mut_ptr(),
            length,
        );

        if ok == 0 {
            return Err(DdcError::CapabilitiesReplyFailed(last_os_error()));
        }

        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        Ok(String::from_utf8_lossy(&buf[..end]).into_owned())
    }
}

pub(super) fn get_capabilities_infos(hmonitor: HMONITOR) -> Result<Vec<CapabilitiesInfo>> {
    let pms = get_physical_monitors(hmonitor)?;
    if pms.is_empty() {
        return Err(DdcError::GetPhysicalMonitorsFailed(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No physical monitors found",
        )));
    }

    let guard = MonitorGuard::new(pms);
    let mut out = Vec::new();

    for (i, pm) in guard.as_slice().iter().enumerate() {
        let friendly = get_friendly_name(pm);
        let raw = get_capabilities_string(pm.hphysicalmonitor)?;
        let parsed = parse_capabilities(&raw)?;
        out.push(CapabilitiesInfo {
            hmonitor,
            pm_index: i,
            friendly_name: friendly,
            raw,
            parsed,
        });
    }

    Ok(out)
}

fn get_vcp_feature_pm(pm_handle: HANDLE, code: u8) -> Option<(u32, u32)> {
    unsafe {
        let mut code_type: MC_VCP_CODE_TYPE = zeroed();
        let mut current = 0u32;
        let mut maximum = 0u32;

        let ok = GetVCPFeatureAndVCPFeatureReply(
            pm_handle,
            code,
            &mut code_type,
            &mut current,
            &mut maximum,
        );

        if ok == 0 {
            None
        } else {
            Some((current, maximum))
        }
    }
}

fn set_vcp_feature_pm(pm_handle: HANDLE, code: u8, value: u32) -> bool {
    unsafe { SetVCPFeature(pm_handle, code, value) != 0 }
}

pub(super) fn get_vcp_feature(hmonitor: HMONITOR, code: u8) -> Result<(u32, u32)> {
    let pms = get_physical_monitors(hmonitor)?;
    if pms.is_empty() {
        return Err(DdcError::GetPhysicalMonitorsFailed(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No physical monitors found",
        )));
    }

    let guard = MonitorGuard::new(pms);
    let mut last_err: Option<std::io::Error> = None;

    for pm in guard.as_slice() {
        if let Some(v) = get_vcp_feature_pm(pm.hphysicalmonitor, code) {
            return Ok(v);
        }
        last_err = Some(last_os_error());
    }

    Err(DdcError::VcpGetFailed(format!(
        "code=0x{code:02X}; last_error={last_err:?}"
    )))
}

pub(super) fn get_vcp_features(
    hmonitor: HMONITOR,
    codes: impl IntoIterator<Item = u8>,
) -> Result<HashMap<u8, (u32, u32)>> {
    let mut norm_codes = Vec::new();
    let mut seen = HashSet::new();

    for code in codes {
        if seen.insert(code) {
            norm_codes.push(code);
        }
    }

    if norm_codes.is_empty() {
        return Ok(HashMap::new());
    }

    let pms = get_physical_monitors(hmonitor)?;
    if pms.is_empty() {
        return Err(DdcError::GetPhysicalMonitorsFailed(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No physical monitors found",
        )));
    }

    let guard = MonitorGuard::new(pms);
    let mut results = HashMap::new();
    let mut last_err: Option<std::io::Error> = None;

    for code in norm_codes {
        let mut got = None;

        for pm in guard.as_slice() {
            if let Some(v) = get_vcp_feature_pm(pm.hphysicalmonitor, code) {
                got = Some(v);
                break;
            }

            last_err = Some(last_os_error());
        }

        if let Some(v) = got {
            results.insert(code, v);
        }
    }

    if results.is_empty() {
        return Err(DdcError::VcpGetFailed(format!(
            "all requested codes failed; last_error={last_err:?}"
        )));
    }

    Ok(results)
}

pub(super) fn set_vcp_feature(hmonitor: HMONITOR, code: u8, value: u32) -> Result<bool> {
    let pms = get_physical_monitors(hmonitor)?;
    if pms.is_empty() {
        return Err(DdcError::GetPhysicalMonitorsFailed(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No physical monitors found",
        )));
    }

    let guard = MonitorGuard::new(pms);
    let mut last_err: Option<std::io::Error> = None;

    for pm in guard.as_slice() {
        if set_vcp_feature_pm(pm.hphysicalmonitor, code, value) {
            return Ok(true);
        }
        last_err = Some(last_os_error());
    }

    Err(DdcError::VcpSetFailed(format!(
        "code=0x{code:02X}; value={value}; last_error={last_err:?}"
    )))
}

pub(super) fn set_vcp_features(
    hmonitor: HMONITOR,
    items: impl IntoIterator<Item = (u8, u32)>,
) -> Result<HashMap<u8, bool>> {
    let mut norm_items = Vec::new();
    let mut seen = HashSet::new();

    for (code, value) in items {
        if seen.insert(code) {
            norm_items.push((code, value));
        }
    }

    if norm_items.is_empty() {
        return Ok(HashMap::new());
    }

    let pms = get_physical_monitors(hmonitor)?;
    if pms.is_empty() {
        return Err(DdcError::GetPhysicalMonitorsFailed(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No physical monitors found",
        )));
    }

    let guard = MonitorGuard::new(pms);
    let mut status = HashMap::new();
    let mut last_err: Option<std::io::Error> = None;

    for (code, value) in norm_items {
        let mut any_ok = false;

        for pm in guard.as_slice() {
            if set_vcp_feature_pm(pm.hphysicalmonitor, code, value) {
                any_ok = true;
            } else {
                last_err = Some(last_os_error());
            }
        }

        status.insert(code, any_ok);
    }

    let failed: Vec<u8> = status
        .iter()
        .filter_map(|(&code, &ok)| if ok { None } else { Some(code) })
        .collect();

    if !failed.is_empty() {
        let failed_hex = failed
            .iter()
            .map(|c| format!("0x{c:02X}"))
            .collect::<Vec<_>>()
            .join(" ");

        return Err(DdcError::VcpSetFailed(format!(
            "codes={failed_hex}; last_error={last_err:?}"
        )));
    }

    Ok(status)
}

pub(super) fn get_vcp_features_for_hmonitor(hmonitor: HMONITOR) -> Result<Vec<VcpFeature>> {
    let infos = get_capabilities_infos(hmonitor)?;
    if infos.is_empty() {
        return Ok(Vec::new());
    }

    let parsed = infos
        .iter()
        .find(|info| !info.parsed.vcp_codes.is_empty())
        .map(|info| &info.parsed)
        .ok_or(DdcError::NoVcpCodeList)?;

    let got = get_vcp_features(hmonitor, parsed.vcp_codes.iter().copied())?;

    let features = parsed
        .vcp_codes
        .iter()
        .filter_map(|&code| {
            got.get(&code).map(|&(current, maximum)| VcpFeature {
                vcp_code: code,
                current,
                maximum,
            })
        })
        .collect();

    Ok(features)
}