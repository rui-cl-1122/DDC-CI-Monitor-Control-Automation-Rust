use thiserror::Error;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredEdid {
    pub identifier: String,
    pub vendor: String,
    pub product_id: u16,
    pub serial: u32,
    pub week: u8,
    pub year: u16,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DdcDiscoveredMonitor {
    pub logical_name: String,
    pub friendly_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdidDiscoveredMonitor {
    pub logical_name: String,
    pub edid: DiscoveredEdid,
}



#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GetMonitorsPortError {
    #[error("monitors not found")]
    MonitorsNotFound,

    #[error("monitor backend unavailable")]
    BackendUnavailable,
}



pub trait GetDdcMonitorsPort: Send + Sync {
    fn get_ddc_monitors(&self) -> Result<Vec<DdcDiscoveredMonitor>, GetMonitorsPortError>;
}


pub trait GetEdidMonitorsPort: Send + Sync {
    fn get_edid_monitors(&self) -> Result<Vec<EdidDiscoveredMonitor>, GetMonitorsPortError>;
}