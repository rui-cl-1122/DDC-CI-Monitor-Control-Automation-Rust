use tauri::State;

use crate::application::monitor::get_monitors::{
    GetMonitorsError,
    GetMonitorsRequest,
    GetMonitorsResponse,
};
use crate::bootstrap::container::AppContainer;


#[tauri::command]
pub fn get_monitors_command(
    container: State<'_, AppContainer>,
) -> Result<GetMonitorsResponse, GetMonitorsError> {
    container
        .get_monitors_use_case() // containerからget_monitors_...を取り出す
        .execute(GetMonitorsRequest::default()) // 引数空なのでdefault()で実行
}

// #[tauri::command]
// 将来のコマンド