#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformBasicEdid {
    pub identifier: String,
    pub vendor: String,
    pub product_id: u16,
    pub serial: u32,
    pub week: u8,
    pub year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformEdidMonitor {
    pub logical_name: String,
    pub edid: PlatformBasicEdid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdidPlatformPortError {
    Unavailable,
}

pub trait EdidPlatformPort: Send + Sync {
    fn list_edid_monitors(&self) -> Result<Vec<PlatformEdidMonitor>, EdidPlatformPortError>;
}