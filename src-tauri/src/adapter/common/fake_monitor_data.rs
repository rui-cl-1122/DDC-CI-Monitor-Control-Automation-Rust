use crate::application::monitor::get_monitors::{
    DiscoveredEdid,
    DdcDiscoveredMonitor,
    EdidDiscoveredMonitor,
};

pub const DISPLAY1_LOGICAL_NAME: &str = r"\\.\DISPLAY1";
pub const DISPLAY2_LOGICAL_NAME: &str = r"\\.\DISPLAY2";

pub const DISPLAY1_FRIENDLY_NAME: &str = "DELL U2720Q";
pub const DISPLAY2_FRIENDLY_NAME: &str = "LG ULTRAWIDE";

pub fn display1_ddc_monitor() -> DdcDiscoveredMonitor {
    DdcDiscoveredMonitor {
        logical_name: DISPLAY1_LOGICAL_NAME.to_string(),
        friendly_name: Some(DISPLAY1_FRIENDLY_NAME.to_string()),
    }
}

pub fn display2_ddc_monitor() -> DdcDiscoveredMonitor {
    DdcDiscoveredMonitor {
        logical_name: DISPLAY2_LOGICAL_NAME.to_string(),
        friendly_name: Some(DISPLAY2_FRIENDLY_NAME.to_string()),
    }
}

pub fn display1_edid_monitor() -> EdidDiscoveredMonitor {
    EdidDiscoveredMonitor {
        logical_name: DISPLAY1_LOGICAL_NAME.to_string(),
        edid: DiscoveredEdid {
            identifier: "DEL-1A2B-2024W11-12345678".to_string(),
            vendor: "DEL".to_string(),
            product_id: 0x1A2B,
            serial: 12345678,
            week: 11,
            year: 2024,
        },
    }
}

pub fn display2_edid_monitor() -> EdidDiscoveredMonitor {
    EdidDiscoveredMonitor {
        logical_name: DISPLAY2_LOGICAL_NAME.to_string(),
        edid: DiscoveredEdid {
            identifier: "GSM-4C2D-2023W09-87654321".to_string(),
            vendor: "GSM".to_string(),
            product_id: 0x4C2D,
            serial: 87654321,
            week: 9,
            year: 2023,
        },
    }
}