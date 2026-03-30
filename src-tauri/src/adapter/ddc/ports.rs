#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformDdcMonitor {
    pub logical_name: String,
    pub friendly_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdcPlatformPortError {
    Unavailable,
}

pub trait DdcPlatformPort: Send + Sync {
    fn list_ddc_monitors(&self) -> Result<Vec<PlatformDdcMonitor>, DdcPlatformPortError>;
}