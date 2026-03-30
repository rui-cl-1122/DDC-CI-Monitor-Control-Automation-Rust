use std::collections::HashSet;
use std::sync::Arc;

use crate::application::monitor::get_monitors::{
    DiscoveredEdid,
    EdidDiscoveredMonitor,
    GetEdidMonitorsPort,
    GetMonitorsPortError,
};

use super::ports::{
    EdidPlatformPort,
    EdidPlatformPortError,
};



pub struct EdidGetMonitorsAdapter {
    platform_port: Arc<dyn EdidPlatformPort>,
}

impl EdidGetMonitorsAdapter {
    pub fn new(platform_port: Arc<dyn EdidPlatformPort>) -> Self {
        Self { platform_port }
    }
}



impl GetEdidMonitorsPort for EdidGetMonitorsAdapter {
    fn get_edid_monitors(&self) -> Result<Vec<EdidDiscoveredMonitor>, GetMonitorsPortError> {
        let infos = self
            .platform_port
            .list_edid_monitors()
            .map_err(map_platform_error)?;

        if infos.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        let mut seen = HashSet::<String>::new();
        let mut monitors = Vec::<EdidDiscoveredMonitor>::new();

        for info in infos {
            let logical_name = normalize_required_string(info.logical_name);
            if logical_name.is_empty() {
                continue;
            }

            if !seen.insert(logical_name.clone()) {
                continue;
            }

            monitors.push(EdidDiscoveredMonitor {
                logical_name,
                edid: DiscoveredEdid {
                    identifier: normalize_required_string(info.edid.identifier),
                    vendor: normalize_required_string(info.edid.vendor),
                    product_id: info.edid.product_id,
                    serial: info.edid.serial,
                    week: info.edid.week,
                    year: info.edid.year,
                },
            });
        }

        if monitors.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        Ok(monitors)
    }
}


fn map_platform_error(err: EdidPlatformPortError) -> GetMonitorsPortError {
    match err {
        EdidPlatformPortError::Unavailable => GetMonitorsPortError::BackendUnavailable,
    }
}


fn normalize_required_string(value: String) -> String {
    value.trim().to_owned()
}