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



            modules::bluetooth::scan_bluetooth,
            modules::bluetooth::enable_bluetooth,
            modules::bluetooth::disable_bluetooth,
            modules::bluetooth::connect_bluetooth_device,
            modules::bluetooth::disconnect_bluetooth_device,
            modules::display::get_brightness,
            modules::display::set_brightness,
            modules::sound::get_input_devices,
            modules::sound::get_output_devices,
            modules::sound::get_output_sound_value,
            modules::sound::set_output_sound_value,
            modules::sound::get_input_sound_value,
            modules::sound::set_input_sound_value,
            modules::sound::input_device_toggle_mute,
            modules::sound::output_device_toggle_mute,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
