#![cfg(windows)]

use crate::infra::winapi::edid::api::{BasicEdidFields, EdidInfo, DISPLAYCONFIG_PATH_INFO};
use crate::infra::winapi::edid::error::{EdidError, Result};

use std::ffi::OsString;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;

use windows_sys::Win32::Devices::Display::{
    DisplayConfigGetDeviceInfo,
    GetDisplayConfigBufferSizes,
    QueryDisplayConfig,
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
    DISPLAYCONFIG_DEVICE_INFO_HEADER,
    DISPLAYCONFIG_MODE_INFO,
    DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    DISPLAYCONFIG_TARGET_DEVICE_NAME,
    QDC_ONLY_ACTIVE_PATHS,
};
use windows_sys::Win32::Foundation::{
    ERROR_FILE_NOT_FOUND,
    ERROR_SUCCESS,
};

use windows_sys::Win32::System::Registry::{
    RegCloseKey,
    RegOpenKeyExW,
    RegQueryValueExW,
    HKEY_LOCAL_MACHINE,
    KEY_READ,
    REG_BINARY,
    HKEY
};

fn format_win32_status(code: u32) -> String {
    std::io::Error::from_raw_os_error(code as i32).to_string()
}

fn wide_nul_terminated_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    OsString::from_wide(&buf[..len])
        .to_string_lossy()
        .into_owned()
}

fn to_wide_null(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(Some(0))
        .collect()
}

struct RegKey(HKEY);

impl Drop for RegKey {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = RegCloseKey(self.0);
            }
        }
    }
}

fn query_display_config_full() -> Result<(Vec<DISPLAYCONFIG_PATH_INFO>, Vec<DISPLAYCONFIG_MODE_INFO>)> {
    let mut path_count = 0u32;
    let mut mode_count = 0u32;

    unsafe {
        let status = GetDisplayConfigBufferSizes(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_count,
            &mut mode_count,
        );

        if status != ERROR_SUCCESS {
            return Err(EdidError::BufferSizeQueryFailed(format!(
                "code={status}; {}",
                format_win32_status(status)
            )));
        }

        let mut paths = vec![zeroed::<DISPLAYCONFIG_PATH_INFO>(); path_count as usize];
        let mut modes = vec![zeroed::<DISPLAYCONFIG_MODE_INFO>(); mode_count as usize];

        let status = QueryDisplayConfig(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_count,
            paths.as_mut_ptr(),
            &mut mode_count,
            modes.as_mut_ptr(),
            null_mut(),
        );

        if status != ERROR_SUCCESS {
            return Err(EdidError::QueryConfigFailed(format!(
                "code={status}; {}",
                format_win32_status(status)
            )));
        }

        paths.truncate(path_count as usize);
        modes.truncate(mode_count as usize);

        Ok((paths, modes))
    }
}

pub(super) fn list_active_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>> {
    let (paths, _) = query_display_config_full()?;
    Ok(paths)
}

pub(super) fn get_logical_display_name(path: &DISPLAYCONFIG_PATH_INFO) -> Result<String> {
    unsafe {
        let mut src_info: DISPLAYCONFIG_SOURCE_DEVICE_NAME = zeroed();
        src_info.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
            r#type: DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
            size: size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32,
            adapterId: path.sourceInfo.adapterId,
            id: path.sourceInfo.id,
        };

        let status = DisplayConfigGetDeviceInfo(&mut src_info.header);
        if status != ERROR_SUCCESS as i32 {
            let code = status as u32;
            return Err(EdidError::GetSourceNameFailed(format!(
                "code={code}; {}",
                format_win32_status(code)
            )));
        }

        Ok(wide_nul_terminated_to_string(&src_info.viewGdiDeviceName))
    }
}

pub(super) fn get_pnp_device_path(path: &DISPLAYCONFIG_PATH_INFO) -> Result<String> {
    unsafe {
        let mut tgt_info: DISPLAYCONFIG_TARGET_DEVICE_NAME = zeroed();
        tgt_info.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
            r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
            size: size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
            adapterId: path.targetInfo.adapterId,
            id: path.targetInfo.id,
        };

        let status = DisplayConfigGetDeviceInfo(&mut tgt_info.header);
        if status != ERROR_SUCCESS as i32 {
            let code = status as u32;
            return Err(EdidError::GetTargetNameFailed(format!(
                "code={code}; {}",
                format_win32_status(code)
            )));
        }

        Ok(wide_nul_terminated_to_string(&tgt_info.monitorDevicePath))
    }
}

pub(super) fn pnp_device_path_to_registry_path(pnp_device_path: &str) -> String {
    let mut reg_path = pnp_device_path.trim_start_matches(r"\\?\").to_string();

    if let Some(pos) = reg_path.find("#{") {
        reg_path.truncate(pos);
    }

    reg_path = reg_path.replace('#', "\\");

    format!(
        "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters",
        reg_path
    )
}

pub(super) fn read_edid_from_registry(registry_path: &str) -> Result<Vec<u8>> {
    unsafe {
        let key_path = to_wide_null(registry_path);
        let mut raw_hkey: HKEY = null_mut();

        let status = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path.as_ptr(),
            0,
            KEY_READ,
            &mut raw_hkey,
        );

        if status != ERROR_SUCCESS {
            return Err(EdidError::RegistryOpenFailed {
                path: registry_path.to_string(),
                message: format!("code={status}; {}", format_win32_status(status)),
            });
        }

        let hkey = RegKey(raw_hkey);

        let value_name = to_wide_null("EDID");
        let mut value_type = 0u32;
        let mut size = 0u32;

        let status = RegQueryValueExW(
            hkey.0,
            value_name.as_ptr(),
            null_mut(),
            &mut value_type,
            null_mut(),
            &mut size,
        );

        if status == ERROR_FILE_NOT_FOUND {
            return Err(EdidError::EdidNotFound(registry_path.to_string()));
        }

        if status != ERROR_SUCCESS {
            return Err(EdidError::RegistryReadFailed {
                path: registry_path.to_string(),
                message: format!("code={status}; {}", format_win32_status(status)),
            });
        }

        if value_type != REG_BINARY {
            return Err(EdidError::RegistryReadFailed {
                path: registry_path.to_string(),
                message: "Unexpected registry value type (expected REG_BINARY)".to_string(),
            });
        }

        let mut edid = vec![0u8; size as usize];

        let status = RegQueryValueExW(
            hkey.0,
            value_name.as_ptr(),
            null_mut(),
            null_mut(),
            edid.as_mut_ptr(),
            &mut size,
        );

        if status != ERROR_SUCCESS {
            return Err(EdidError::RegistryReadFailed {
                path: registry_path.to_string(),
                message: format!("code={status}; {}", format_win32_status(status)),
            });
        }

        edid.truncate(size as usize);
        Ok(edid)
    }
}

pub(super) fn parse_basic_edid_fields(edid: &[u8]) -> Result<BasicEdidFields> {
    if edid.len() < 128 {
        return Err(EdidError::InvalidEdid(format!(
            "EDID too short ({} bytes). Need at least 128 bytes.",
            edid.len()
        )));
    }

    let manufacturer_id = ((edid[8] as u16) << 8) | edid[9] as u16;

    let vendor: String = [
        ((manufacturer_id >> 10) & 0x1F) as u8,
        ((manufacturer_id >> 5) & 0x1F) as u8,
        (manufacturer_id & 0x1F) as u8,
    ]
    .iter()
    .map(|&c| {
        if c == 0 {
            '?'
        } else {
            (b'A' + c - 1) as char
        }
    })
    .collect();

    let product_id = u16::from_le_bytes([edid[10], edid[11]]);
    let serial = u32::from_le_bytes([edid[12], edid[13], edid[14], edid[15]]);
    let week = match edid[16] {
        0 | 255 => 0,
        w => w,
    };
    let year = edid[17] as u16 + 1990;

    let identifier = format!("{vendor}-{product_id:04X}-{year}W{week:02}-{serial}");

    Ok(BasicEdidFields {
        identifier,
        vendor,
        product_id,
        serial,
        week,
        year,
    })
}

pub(super) fn get_edid_info(path: &DISPLAYCONFIG_PATH_INFO) -> Result<EdidInfo> {
    let logical_name = get_logical_display_name(path)?;
    let pnp_device_path = get_pnp_device_path(path)?;
    let registry_path = pnp_device_path_to_registry_path(&pnp_device_path);
    let raw_edid = read_edid_from_registry(&registry_path)?;
    let parsed = parse_basic_edid_fields(&raw_edid)?;

    Ok(EdidInfo {
        logical_name,
        pnp_device_path,
        registry_path,
        raw_edid,
        parsed,
    })
}

pub(super) fn list_edid_infos() -> Result<Vec<EdidInfo>> {
    let mut out = Vec::new();

    for path in list_active_paths()? {
        out.push(get_edid_info(&path)?);
    }

    Ok(out)
}