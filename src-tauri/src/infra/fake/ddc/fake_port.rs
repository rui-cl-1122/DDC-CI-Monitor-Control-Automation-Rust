use crate::adapter::ddc::ports::{
    DdcPlatformPort,
    DdcPlatformPortError,
    PlatformDdcMonitor,
};

use super::fake_monitor_data::{
    display1_ddc_monitor,
    display2_ddc_monitor,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdcFakeScenario {
    Success,
    FriendlyNameMissingOnDisplay1,
    FriendlyNameMissingOnDisplay2,
    FriendlyNameMissingOnBoth,
    NoMonitors,
    BackendUnavailable,
}

pub struct FakeDdcPlatformPort {
    scenario: DdcFakeScenario,
}

impl FakeDdcPlatformPort {
    pub fn new(scenario: DdcFakeScenario) -> Self {
        Self { scenario }
    }
}

impl DdcPlatformPort for FakeDdcPlatformPort {
    fn list_ddc_monitors(&self) -> Result<Vec<PlatformDdcMonitor>, DdcPlatformPortError> {
        match self.scenario {
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
            
            DdcFakeScenario::NoMonitors => Ok(vec![]),
            
            DdcFakeScenario::BackendUnavailable => {
                Err(DdcPlatformPortError::Unavailable)
            }
        }
    }
}