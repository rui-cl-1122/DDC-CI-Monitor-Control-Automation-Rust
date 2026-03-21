use std::sync::Arc;

use crate::adapter::ddc::get_monitors_adapter::DdcGetMonitorsAdapter;
use crate::adapter::edid::get_monitors_adapter::EdidGetMonitorsAdapter;
use crate::application::monitor::get_monitors::{
    GetDdcMonitorsPort,
    GetEdidMonitorsPort,
    GetMonitorsUseCase,
};


// FakeAdapter実装
// use crate::adapter::ddc::fake_get_monitors_adapter::FakeDdcGetMonitorsAdapter;
// use crate::adapter::edid::fake_get_monitors_adapter::FakeEdidGetMonitorsAdapter;



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



pub fn build_app_container() -> AppContainer {
    
    // Adapterを接続
    let get_ddc_monitors_port: Arc<dyn GetDdcMonitorsPort> =
        Arc::new(DdcGetMonitorsAdapter::new());

    let get_edid_monitors_port: Arc<dyn GetEdidMonitorsPort> =
        Arc::new(EdidGetMonitorsAdapter::new());

    // UseCaseを接続
    let get_monitors_use_case = GetMonitorsUseCase::new(
        get_ddc_monitors_port,
        get_edid_monitors_port,
    );

    // Containerに詰める
    AppContainer::new(get_monitors_use_case)
}