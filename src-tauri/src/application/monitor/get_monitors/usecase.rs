use std::sync::Arc;

use super::usecase_impl;

use super::{
    GetDdcMonitorsPort,
    GetEdidMonitorsPort,
    GetMonitorsError,
    GetMonitorsRequest,
    GetMonitorsResponse,
};

pub struct GetMonitorsUseCase {
    ddc_port: Arc<dyn GetDdcMonitorsPort>,
    edid_port: Arc<dyn GetEdidMonitorsPort>,
}

/// edidが取得失敗時は空扱いで続行
impl GetMonitorsUseCase {
    pub fn new(
        ddc_port: Arc<dyn GetDdcMonitorsPort>,
        edid_port: Arc<dyn GetEdidMonitorsPort>,
    ) -> Self {
        Self { ddc_port, edid_port }
    }

    pub fn execute(
        &self,
        _req: GetMonitorsRequest,
    ) -> Result<GetMonitorsResponse, GetMonitorsError> {
        let ddc_monitors = self
            .ddc_port
            .get_ddc_monitors()
            .map_err(usecase_impl::map_port_error)?;

        let edid_monitors = self
            .edid_port
            .get_edid_monitors()
            .unwrap_or_else(|_| Vec::new());

        let edid_map = usecase_impl::build_edid_map(edid_monitors);

        let monitors = ddc_monitors
            .into_iter()
            .map(|monitor| usecase_impl::map_monitor_identity(monitor, &edid_map))
            .collect::<Vec<_>>();

        Ok(GetMonitorsResponse { monitors })
    }
}