use crate::adapter::edid::ports::{
    EdidPlatformPort,
    EdidPlatformPortError,
    PlatformEdidMonitor,
};

use crate::infra::fake::common::fake_monitor_data::{
    display1_edid_monitor,
    display2_edid_monitor,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdidFakeScenario {
    Success,
    EdidMissingOnDisplay1,
    EdidMissingOnDisplay2,
    EdidMissingOnBoth,
    NoMonitors,
    BackendUnavailable,
}

pub struct FakeEdidPlatformPort {
    scenario: EdidFakeScenario,
}

impl FakeEdidPlatformPort {
    pub fn new(scenario: EdidFakeScenario) -> Self {
        Self { scenario }
    }
}

impl EdidPlatformPort for FakeEdidPlatformPort {
    fn list_edid_monitors(&self) -> Result<Vec<PlatformEdidMonitor>, EdidPlatformPortError> {
        match self.scenario {
            EdidFakeScenario::Success => Ok(vec![
                display1_edid_monitor(),
                display2_edid_monitor(),
            ]),

            EdidFakeScenario::EdidMissingOnDisplay1 => Ok(vec![
                display2_edid_monitor(),
            ]),

            EdidFakeScenario::EdidMissingOnDisplay2 => Ok(vec![
                display1_edid_monitor(),
            ]),

            EdidFakeScenario::EdidMissingOnBoth => Ok(vec![]),

            EdidFakeScenario::NoMonitors => Ok(vec![]),

            EdidFakeScenario::BackendUnavailable => {
                Err(EdidPlatformPortError::Unavailable)
            }
        }
    }
}