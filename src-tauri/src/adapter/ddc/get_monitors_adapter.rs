use std::collections::HashSet;
use std::sync::Arc;

use crate::application::monitor::get_monitors::{
    DdcDiscoveredMonitor,
    GetDdcMonitorsPort,
    GetMonitorsPortError,
};

use super::ports::{
    DdcPlatformPort,
    DdcPlatformPortError,
};



pub struct DdcGetMonitorsAdapter {
    platform_port: Arc<dyn DdcPlatformPort>,
}


impl DdcGetMonitorsAdapter {
    pub fn new(platform_port: Arc<dyn DdcPlatformPort>) -> Self {
        Self { platform_port }
    }
}


impl GetDdcMonitorsPort for DdcGetMonitorsAdapter {
    fn get_ddc_monitors(&self) -> Result<Vec<DdcDiscoveredMonitor>, GetMonitorsPortError> {
        let infos = self
            .platform_port
            .list_ddc_monitors()
            .map_err(map_platform_error)?;

        if infos.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        let mut seen = HashSet::<String>::new();
        let mut monitors = Vec::<DdcDiscoveredMonitor>::new();

        for info in infos {
            let logical_name = normalize_required_string(info.logical_name);
            if logical_name.is_empty() {
                continue;
            }

            if !seen.insert(logical_name.clone()) {
                continue;
            }

            let friendly_name = normalize_optional_string(info.friendly_name);

            monitors.push(DdcDiscoveredMonitor {
                logical_name,
                friendly_name,
            });
        }

        if monitors.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        Ok(monitors)
    }
}


fn map_platform_error(err: DdcPlatformPortError) -> GetMonitorsPortError {
    match err {
        DdcPlatformPortError::Unavailable => GetMonitorsPortError::BackendUnavailable,
    }
}


fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
}


fn normalize_required_string(value: String) -> String {
    value.trim().to_owned()
}