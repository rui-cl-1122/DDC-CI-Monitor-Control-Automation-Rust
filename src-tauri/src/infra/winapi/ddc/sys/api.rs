#![cfg(windows)]

use crate::infra::winapi::ddc::sys::caps_parse::CapabilitiesParsed;
use crate::infra::winapi::ddc::sys::error::Result;

use std::collections::HashMap;

pub use windows_sys::Win32::Graphics::Gdi::HMONITOR;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VcpFeature {
    pub vcp_code: u8,
    pub current: u32,
    pub maximum: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMonitorInfo {
    pub hmonitor: HMONITOR,
    pub pm_index: usize,
    pub device_name: String,
    pub friendly_name: Option<String>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitiesInfo {
    pub hmonitor: HMONITOR,
    pub pm_index: usize,
    pub friendly_name: Option<String>,
    pub raw: String,
    pub parsed: CapabilitiesParsed,
}





/// システム上の HMONITOR を列挙
pub fn enum_hmonitors() -> Result<Vec<HMONITOR>> {
    super::imp::enum_hmonitors()
}


/// HMONITOR から `\\.\DISPLAYx` 名を取得
pub fn get_device_name(hmonitor: HMONITOR) -> Option<String> {
    super::imp::get_device_name(hmonitor)
}


/// 全 physical monitor 情報を列挙 取得時間は早い(数百µs)
/// - HMONITOR → physical monitor に展開される
pub fn list_physical_monitor_infos() -> Result<Vec<PhysicalMonitorInfo>> {
    super::imp::list_physical_monitor_infos()
}





/// 指定モニタ1つ分 HMONITOR 配下の capabilities を取得
/// - physical monitor ごとに1件ずつ返る
/// - 取得は遅い(数秒)
pub fn get_capabilities_infos(hmonitor: HMONITOR) -> Result<Vec<CapabilitiesInfo>> {
    super::imp::get_capabilities_infos(hmonitor)
}


/// 単一 VCP code の current / max を取得
/// - 複数 physical monitor がある場合、最初に成功した値を返す
/// - 戻り値は `(current, maximum)`
pub fn get_vcp_feature(hmonitor: HMONITOR, code: u8) -> Result<(u32, u32)> {
    super::imp::get_vcp_feature(hmonitor, code)
}


/// 指定した複数 VCP code の current / max をまとめて取得
/// - code ごとに最初に成功した値のみ返す
/// - 取得できなかった code は結果に含まれない
pub fn get_vcp_features(
    hmonitor: HMONITOR,
    codes: impl IntoIterator<Item = u8>,
) -> Result<HashMap<u8, (u32, u32)>> {
    super::imp::get_vcp_features(hmonitor, codes)
}


/// モニタ1つのcapabilities から取得可能な 全VCP code を列挙し、
/// それぞれの current / max を取得
/// - 取得できない code は結果に含まれない
/// - 取得は遅い(数秒)
pub fn get_vcp_features_for_hmonitor(hmonitor: HMONITOR) -> Result<Vec<VcpFeature>> {
    super::imp::get_vcp_features_for_hmonitor(hmonitor)
}





/// 単一 VCP code を設定
/// - 複数 physical monitor のうちどれか1つ成功すれば OK
pub fn set_vcp_feature(hmonitor: HMONITOR, code: u8, value: u32) -> Result<bool> {
    super::imp::set_vcp_feature(hmonitor, code, value)
}


/// 複数 VCP code をまとめて設定
/// - 各 code ごとに「どれか成功」で true
/// - 1つでも全失敗の code があれば Err
pub fn set_vcp_features(
    hmonitor: HMONITOR,
    items: impl IntoIterator<Item = (u8, u32)>,
) -> Result<HashMap<u8, bool>> {
    super::imp::set_vcp_features(hmonitor, items)
}