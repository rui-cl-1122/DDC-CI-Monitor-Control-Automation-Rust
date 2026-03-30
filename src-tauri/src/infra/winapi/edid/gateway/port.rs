use crate::adapter::edid::ports::{
    EdidPlatformPort,
    EdidPlatformPortError,
    PlatformBasicEdid,
    PlatformEdidMonitor,
};

use crate::infra::winapi::edid::sys::api::list_edid_infos;

pub struct WinApiEdidPlatformPort;

impl WinApiEdidPlatformPort {
    pub fn new() -> Self {
        Self
    }
}

impl EdidPlatformPort for WinApiEdidPlatformPort {
    
    fn list_edid_monitors(&self) -> Result<Vec<PlatformEdidMonitor>, EdidPlatformPortError> {
        let infos = list_edid_infos()
            .map_err(|_| EdidPlatformPortError::Unavailable)?;

        let monitors = infos
            .into_iter()
            .map(|info| PlatformEdidMonitor {
                logical_name: info.logical_name,
                edid: PlatformBasicEdid {
                    identifier: info.parsed.identifier,
                    vendor: info.parsed.vendor,
                    product_id: info.parsed.product_id,
                    serial: info.parsed.serial,
                    week: info.parsed.week,
                    year: info.parsed.year,
                },
            })
            .collect::<Vec<_>>();

        Ok(monitors)
    }
}