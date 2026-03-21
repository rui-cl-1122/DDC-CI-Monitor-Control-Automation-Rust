use crate::adapter::common::fake_monitor_data::{
    display1_edid_monitor,
    display2_edid_monitor,
};
use crate::application::monitor::get_monitors::{
    EdidDiscoveredMonitor,
    GetEdidMonitorsPort,
    GetMonitorsPortError,
};

// シナリオを1つ有効にして実行

// const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::Success;
const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::EdidMissingOnDisplay1;
// const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::EdidMissingOnDisplay2;
// const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::EdidMissingOnBoth;
// const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::MonitorsNotFound;
// const EDID_FAKE_SCENARIO: EdidFakeScenario = EdidFakeScenario::BackendUnavailable;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdidFakeScenario {
    Success,
    EdidMissingOnDisplay1,
    EdidMissingOnDisplay2,
    EdidMissingOnBoth,
    MonitorsNotFound,
    BackendUnavailable,
}

pub struct FakeEdidGetMonitorsAdapter;

impl FakeEdidGetMonitorsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl GetEdidMonitorsPort for FakeEdidGetMonitorsAdapter {
    fn get_edid_monitors(&self) -> Result<Vec<EdidDiscoveredMonitor>, GetMonitorsPortError> {
        match EDID_FAKE_SCENARIO {
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

            EdidFakeScenario::MonitorsNotFound => Err(GetMonitorsPortError::MonitorsNotFound),

            EdidFakeScenario::BackendUnavailable => Err(GetMonitorsPortError::BackendUnavailable),
        }
    }
}