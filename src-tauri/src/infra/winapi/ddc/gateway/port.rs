use crate::adapter::ddc::ports::{
    DdcPlatformPort,
    DdcPlatformPortError,
    PlatformDdcMonitor,
};

use crate::infra::winapi::ddc::sys::api::list_physical_monitor_infos;

pub struct WinApiDdcPlatformPort;

impl WinApiDdcPlatformPort {
    pub fn new() -> Self {
        Self
    }
}

impl DdcPlatformPort for WinApiDdcPlatformPort {
    fn list_ddc_monitors(&self) -> Result<Vec<PlatformDdcMonitor>, DdcPlatformPortError> {
        let infos = list_physical_monitor_infos()
            .map_err(|_| DdcPlatformPortError::Unavailable)?;

        let monitors = infos
            .into_iter()
            .map(|info| PlatformDdcMonitor {
                logical_name: info.device_name,
                friendly_name: info.friendly_name,
            })
            .collect::<Vec<_>>();

        Ok(monitors)
    }
}