use thiserror::Error;

pub type Result<T> = std::result::Result<T, DdcError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct CapabilitiesParseError {
    pub message: String,
}

impl CapabilitiesParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum DdcError {
    #[error("EnumDisplayMonitors failed: {0}")]
    EnumDisplayFailed(std::io::Error),

    #[error("GetPhysicalMonitorsFromHMONITOR failed: {0}")]
    GetPhysicalMonitorsFailed(std::io::Error),

    #[error("GetCapabilitiesStringLength failed: {0}")]
    GetCapabilitiesFailed(std::io::Error),

    #[error("CapabilitiesRequestAndCapabilitiesReply failed: {0}")]
    CapabilitiesReplyFailed(std::io::Error),

    #[error("No vcp codes found in capabilities")]
    NoVcpCodeList,

    #[error("GetVCPFeatureAndVCPFeatureReply failed: {0}")]
    VcpGetFailed(String),

    #[error("SetVCPFeature failed: {0}")]
    VcpSetFailed(String),

    #[error("Capabilities parse failed: {0}")]
    ParseFailed(#[from] CapabilitiesParseError),
}