use thiserror::Error;

pub type Result<T> = std::result::Result<T, EdidError>;

#[derive(Debug, Error)]
pub enum EdidError {
    #[error("GetDisplayConfigBufferSizes failed: {0}")]
    BufferSizeQueryFailed(String),

    #[error("QueryDisplayConfig failed: {0}")]
    QueryConfigFailed(String),

    #[error("DisplayConfigGetDeviceInfo(source name) failed: {0}")]
    GetSourceNameFailed(String),

    #[error("DisplayConfigGetDeviceInfo(target name) failed: {0}")]
    GetTargetNameFailed(String),

    #[error("RegOpenKeyExW failed: path={path}; {message}")]
    RegistryOpenFailed {
        path: String,
        message: String,
    },

    #[error("RegQueryValueExW failed: path={path}; {message}")]
    RegistryReadFailed {
        path: String,
        message: String,
    },

    #[error("EDID not found in registry: {0}")]
    EdidNotFound(String),

    #[error("Invalid EDID: {0}")]
    InvalidEdid(String),
}