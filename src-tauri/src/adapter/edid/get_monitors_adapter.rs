use std::collections::HashSet;

use crate::application::monitor::get_monitors::{
    DiscoveredEdid,
    EdidDiscoveredMonitor,
    GetEdidMonitorsPort,
    GetMonitorsPortError,
};
use crate::infra::winapi::edid::api::list_edid_infos;

pub struct EdidGetMonitorsAdapter;

impl EdidGetMonitorsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl GetEdidMonitorsPort for EdidGetMonitorsAdapter {
    fn get_edid_monitors(&self) -> Result<Vec<EdidDiscoveredMonitor>, GetMonitorsPortError> {
        let infos = list_edid_infos()
            .map_err(|_| GetMonitorsPortError::BackendUnavailable)?;

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

            let parsed = info.parsed;

            monitors.push(EdidDiscoveredMonitor {
                logical_name,
                edid: DiscoveredEdid {
                    identifier: normalize_required_string(parsed.identifier),
                    vendor: normalize_required_string(parsed.vendor),
                    product_id: parsed.product_id,
                    serial: parsed.serial,
                    week: parsed.week,
                    year: parsed.year,
                },
            });
        }

        if monitors.is_empty() {
            return Err(GetMonitorsPortError::MonitorsNotFound);
        }

        Ok(monitors)
    }
}

fn normalize_required_string(value: String) -> String {
    value.trim().to_owned()
}