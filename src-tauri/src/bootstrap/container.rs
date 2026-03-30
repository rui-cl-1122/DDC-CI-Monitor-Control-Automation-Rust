use std::sync::Arc;

use crate::infra::winapi::ddc::gateway::port::WinApiDdcPlatformPort;
use crate::infra::winapi::edid::gateway::port::WinApiEdidPlatformPort;

use crate::adapter::ddc::get_monitors_adapter::DdcGetMonitorsAdapter;
use crate::adapter::ddc::ports::DdcPlatformPort;
use crate::adapter::edid::get_monitors_adapter::EdidGetMonitorsAdapter;
use crate::adapter::edid::ports::EdidPlatformPort;

use crate::application::monitor::get_monitors::{
    GetDdcMonitorsPort,
    GetEdidMonitorsPort,
    GetMonitorsUseCase,
};

#[cfg(feature = "fake")]
use crate::infra::fake::ddc::fake_port::{
    DdcFakeScenario,
    FakeDdcPlatformPort,
};
#[cfg(feature = "fake")]
use crate::infra::fake::edid::fake_port::{
    EdidFakeScenario,
    FakeEdidPlatformPort,
};


/// アプリ全体の「機能セット」をまとめたコンテナ
pub struct AppContainer {
    /// モニタ一覧取得ユースケース
    get_monitors_use_case: GetMonitorsUseCase,
}

impl AppContainer {
    /// すでに構築済みのUseCaseを受け取って格納するだけ
    pub fn new(get_monitors_use_case: GetMonitorsUseCase) -> Self {
        Self { get_monitors_use_case }
    }

    /// GUI側（Tauri Command）からUseCaseを取り出すための窓口
    pub fn get_monitors_use_case(&self) -> &GetMonitorsUseCase {
        &self.get_monitors_use_case
    }
}


/// 通常ビルド時は本番配線を返す
#[cfg(not(feature = "fake"))]
pub fn build_container() -> AppContainer {
    build_app_container()
}


/// fake feature 有効時は fake 配線を返す
#[cfg(feature = "fake")]
pub fn build_container() -> AppContainer {
    build_fake_app_container()
}



fn build_app_container() -> AppContainer {

    let ddc_platform_port: Arc<dyn DdcPlatformPort> =
        Arc::new(WinApiDdcPlatformPort::new());

    let edid_platform_port: Arc<dyn EdidPlatformPort> =
        Arc::new(WinApiEdidPlatformPort::new());

    let get_ddc_monitors_port: Arc<dyn GetDdcMonitorsPort> =
        Arc::new(DdcGetMonitorsAdapter::new(ddc_platform_port));

    let get_edid_monitors_port: Arc<dyn GetEdidMonitorsPort> =
        Arc::new(EdidGetMonitorsAdapter::new(edid_platform_port));

    let get_monitors_use_case = GetMonitorsUseCase::new(
        get_ddc_monitors_port,
        get_edid_monitors_port,
    );

    AppContainer::new(get_monitors_use_case)
}



#[cfg(feature = "fake")]
fn build_fake_app_container() -> AppContainer {
    let ddc_platform_port: Arc<dyn DdcPlatformPort> =
        Arc::new(FakeDdcPlatformPort::new(
            DdcFakeScenario::FriendlyNameMissingOnDisplay1,
        ));

    let edid_platform_port: Arc<dyn EdidPlatformPort> =
        Arc::new(FakeEdidPlatformPort::new(
            EdidFakeScenario::EdidMissingOnDisplay1,
        ));

    let get_ddc_monitors_port: Arc<dyn GetDdcMonitorsPort> =
        Arc::new(DdcGetMonitorsAdapter::new(ddc_platform_port));

    let get_edid_monitors_port: Arc<dyn GetEdidMonitorsPort> =
        Arc::new(EdidGetMonitorsAdapter::new(edid_platform_port));

    let get_monitors_use_case = GetMonitorsUseCase::new(
        get_ddc_monitors_port,
        get_edid_monitors_port,
    );

    AppContainer::new(get_monitors_use_case)
}