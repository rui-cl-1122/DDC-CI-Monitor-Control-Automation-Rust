pub mod infra;
pub mod adapter;
pub mod application;
pub mod bootstrap;
pub mod presentation;

use bootstrap::container::build_app_container;
use presentation::gui::tauri::commands::get_monitors_command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_container = build_app_container();

    tauri::Builder::default()
        .manage(app_container)
        .invoke_handler(tauri::generate_handler![
            get_monitors_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}