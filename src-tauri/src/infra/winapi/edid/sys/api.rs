#![cfg(windows)]

use crate::infra::winapi::edid::sys::error::Result;

pub use windows_sys::Win32::Devices::Display::DISPLAYCONFIG_PATH_INFO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicEdidFields {
    pub identifier: String,
    pub vendor: String,
    pub product_id: u16,
    pub serial: u32,
    pub week: u8,
    pub year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdidInfo {
    pub logical_name: String,
    pub pnp_device_path: String,
    pub registry_path: String,
    pub raw_edid: Vec<u8>,
    pub parsed: BasicEdidFields,
}

/// active な display path を列挙
pub fn list_active_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>> {
    super::imp::list_active_paths()
}

/// path から論理ディスプレイ名 (`\\.\DISPLAYx`) を取得
pub fn get_logical_display_name(path: &DISPLAYCONFIG_PATH_INFO) -> Result<String> {
    super::imp::get_logical_display_name(path)
}

/// path から monitor device path を取得
pub fn get_pnp_device_path(path: &DISPLAYCONFIG_PATH_INFO) -> Result<String> {
    super::imp::get_pnp_device_path(path)
}

/// PnP device path を registry path に変換
pub fn pnp_device_path_to_registry_path(pnp_device_path: &str) -> String {
    super::imp::pnp_device_path_to_registry_path(pnp_device_path)
}

/// registry から raw EDID を取得
pub fn read_edid_from_registry(registry_path: &str) -> Result<Vec<u8>> {
    super::imp::read_edid_from_registry(registry_path)
}

/// raw EDID から基本項目を解析
pub fn parse_basic_edid_fields(edid: &[u8]) -> Result<BasicEdidFields> {
    super::imp::parse_basic_edid_fields(edid)
}

/// path 1件分の EDID 情報を取得
pub fn get_edid_info(path: &DISPLAYCONFIG_PATH_INFO) -> Result<EdidInfo> {
    super::imp::get_edid_info(path)
}

/// active path 全件ぶんの EDID 情報を取得
/// - path ごとに 1件ずつ返る
pub fn list_edid_infos() -> Result<Vec<EdidInfo>> {
    super::imp::list_edid_infos()
}