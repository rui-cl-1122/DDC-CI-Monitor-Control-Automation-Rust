use crate::adapter::common::fake_monitor_data::{
    display1_ddc_monitor,
    display2_ddc_monitor,
};
use crate::application::monitor::get_monitors::{
    DdcDiscoveredMonitor,
    GetDdcMonitorsPort,
    GetMonitorsPortError,
};


// シナリオを1つ有効にして実行

//const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::Success;
const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::FriendlyNameMissingOnDisplay1;
// const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::FriendlyNameMissingOnDisplay2;
// const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::FriendlyNameMissingOnBoth;
// const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::MonitorsNotFound;
// const DDC_FAKE_SCENARIO: DdcFakeScenario = DdcFakeScenario::BackendUnavailable;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DdcFakeScenario {
    Success,
    FriendlyNameMissingOnDisplay1,
    FriendlyNameMissingOnDisplay2,
    FriendlyNameMissingOnBoth,
    MonitorsNotFound,
    BackendUnavailable,
}

pub struct FakeDdcGetMonitorsAdapter;

impl FakeDdcGetMonitorsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl GetDdcMonitorsPort for FakeDdcGetMonitorsAdapter {
    fn get_ddc_monitors(&self) -> Result<Vec<DdcDiscoveredMonitor>, GetMonitorsPortError> {
        match DDC_FAKE_SCENARIO {
            DdcFakeScenario::Success => Ok(vec![
                display1_ddc_monitor(),
                display2_ddc_monitor(),
            ]),

            DdcFakeScenario::FriendlyNameMissingOnDisplay1 => {
                let mut monitor1 = display1_ddc_monitor();
                let monitor2 = display2_ddc_monitor();

                monitor1.friendly_name = None;

                Ok(vec![monitor1, monitor2])
            }

            DdcFakeScenario::FriendlyNameMissingOnDisplay2 => {
                let monitor1 = display1_ddc_monitor();
                let mut monitor2 = display2_ddc_monitor();

                monitor2.friendly_name = None;

                Ok(vec![monitor1, monitor2])
            }

            DdcFakeScenario::FriendlyNameMissingOnBoth => {
                let mut monitor1 = display1_ddc_monitor();
                let mut monitor2 = display2_ddc_monitor();

                monitor1.friendly_name = None;
                monitor2.friendly_name = None;

                Ok(vec![monitor1, monitor2])
            }

            DdcFakeScenario::MonitorsNotFound => Err(GetMonitorsPortError::MonitorsNotFound),

            DdcFakeScenario::BackendUnavailable => Err(GetMonitorsPortError::BackendUnavailable),
        }
    }
}