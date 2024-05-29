// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
mod error;

#[tauri::command]
fn exit_app() {
    std::process::exit(0x0);
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            modules::bluetooth::get_bluetooth_status,
            modules::bluetooth::scan_bluetooth,
            modules::bluetooth::update_enable_bluetooth,
            modules::bluetooth::update_disable_bluetooth,
            modules::bluetooth::connect_bluetooth_device,
            modules::bluetooth::disconnect_bluetooth_device,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
