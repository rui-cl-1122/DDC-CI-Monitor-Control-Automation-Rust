use crate::adapter::ddc::ports::PlatformDdcMonitor;

pub const DISPLAY1_LOGICAL_NAME: &str = r"\\.\DISPLAY1";
pub const DISPLAY2_LOGICAL_NAME: &str = r"\\.\DISPLAY2";

pub const DISPLAY1_FRIENDLY_NAME: &str = "DELL U2720Q";
pub const DISPLAY2_FRIENDLY_NAME: &str = "LG ULTRAWIDE";

pub fn display1_ddc_monitor() -> PlatformDdcMonitor {
    PlatformDdcMonitor {
        logical_name: DISPLAY1_LOGICAL_NAME.to_string(),
        friendly_name: Some(DISPLAY1_FRIENDLY_NAME.to_string()),
    }
}

pub fn display2_ddc_monitor() -> PlatformDdcMonitor {
    PlatformDdcMonitor {
        logical_name: DISPLAY2_LOGICAL_NAME.to_string(),
        friendly_name: Some(DISPLAY2_FRIENDLY_NAME.to_string()),
    }
}