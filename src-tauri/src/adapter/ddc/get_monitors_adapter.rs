use std::collections::HashSet;

use crate::application::monitor::get_monitors::{
    DdcDiscoveredMonitor,
    GetDdcMonitorsPort,
    GetMonitorsPortError,
};
use crate::infra::winapi::ddc::api::list_physical_monitor_infos;

pub struct DdcGetMonitorsAdapter;

impl DdcGetMonitorsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl GetDdcMonitorsPort for DdcGetMonitorsAdapter {
    fn get_ddc_monitors(&self) -> Result<Vec<DdcDiscoveredMonitor>, GetMonitorsPortError> {
        let infos = list_physical_monitor_infos()
            .map_err(|_| GetMonitorsPortError::BackendUnavailable)?;

        if infos.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        let mut seen = HashSet::<String>::new();
        let mut monitors = Vec::<DdcDiscoveredMonitor>::new();

        for info in infos {
            let logical_name = normalize_required_string(info.device_name);
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

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
}

fn normalize_required_string(value: String) -> String {
    value.trim().to_owned()
}