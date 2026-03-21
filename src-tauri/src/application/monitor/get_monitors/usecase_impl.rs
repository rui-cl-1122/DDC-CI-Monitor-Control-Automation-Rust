use std::collections::HashMap;

use super::{
    DiscoveredEdid,
    DdcDiscoveredMonitor,
    EdidDiscoveredMonitor,
    EdidSummary,
    GetMonitorsError,
    GetMonitorsPortError,
    MonitorIdentity,
};

pub(super) fn build_edid_map(
    edid_monitors: Vec<EdidDiscoveredMonitor>,
) -> HashMap<String, DiscoveredEdid> {
    let mut map = HashMap::new();

    for monitor in edid_monitors {
        let logical_name = normalize_required_string(monitor.logical_name);
        if logical_name.is_empty() {
            continue;
        }

        map.entry(logical_name).or_insert(monitor.edid);
    }

    map
}

pub(super) fn map_monitor_identity(
    monitor: DdcDiscoveredMonitor,
    edid_map: &HashMap<String, DiscoveredEdid>,
) -> MonitorIdentity {
    let monitor_id = normalize_required_string(monitor.logical_name);
    let friendly_name = normalize_optional_string(monitor.friendly_name);
    let edid = edid_map
        .get(&monitor_id)
        .cloned()
        .map(map_edid_summary);

    MonitorIdentity {
        monitor_id,
        friendly_name,
        edid,
    }
}

fn map_edid_summary(edid: DiscoveredEdid) -> EdidSummary {
    EdidSummary {
        identifier: normalize_required_string(edid.identifier),
        vendor: normalize_required_string(edid.vendor),
        product_id: edid.product_id,
        serial: edid.serial,
        week: edid.week,
        year: edid.year,
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

pub(super) fn map_port_error(err: GetMonitorsPortError) -> GetMonitorsError {
    match err {
        GetMonitorsPortError::MonitorsNotFound => GetMonitorsError::MonitorsNotFound,
        GetMonitorsPortError::BackendUnavailable => GetMonitorsError::Unavailable,
    }
}