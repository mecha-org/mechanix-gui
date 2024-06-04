// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::JoinHandle, time::Duration};

use tauri::{Manager, Window};

mod error;
mod modules;

#[tauri::command]
fn exit_app() {
    std::process::exit(0x0);
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}


fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
            modules::bluetooth::get_bluetooth_status,


            modules::wireless::get_wireless_status,
            modules::wireless::enable_wifi,
            modules::wireless::disable_wifi,
            modules::wireless::get_wifi_status,
            modules::wireless::get_connected_wifi_info,
            modules::wireless::wifi_scanning,
            modules::wireless::get_known_networks,
            modules::wireless::connect_to_network,
            modules::wireless::connect_to_known_network,



            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
