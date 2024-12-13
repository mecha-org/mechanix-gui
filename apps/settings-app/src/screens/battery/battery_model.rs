use lazy_static::lazy_static;
use mctk_core::{context::Context, font_cache::TextSegment, txt};
use mctk_macros::Model;
use mechanix_desktop_dbus_client::power::Power;
use serde::de::value;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref BATTERY_MODEL: BatteryModel = BatteryModel {
        battery_percentage: Context::new(0.0),
        available_modes: Context::new(vec![]),
        cureent_mode: Context::new("".to_string()),
    };
}

#[derive(Model)]
pub struct BatteryModel {
    pub battery_percentage: Context<f64>,
    pub available_modes: Context<Vec<String>>,
    pub cureent_mode: Context<String>,
}

impl BatteryModel {
    pub fn get() -> &'static Self {
        &BATTERY_MODEL
    }

    pub fn update() {
        RUNTIME.spawn(async {
            let battery = upower::get_battery().await.unwrap();
            let percentage = battery.percentage().await.unwrap();
            BatteryModel::get().battery_percentage.set(percentage);

            let mut available_modes: Vec<String> = Power::get_available_governors().await.unwrap();
            for mode in available_modes.iter_mut() {
                if *mode == "performance" {
                    *mode = "High".to_string();
                } else if *mode == "powersave" {
                    *mode = "Low".to_string();
                } else if *mode == "conservative" {
                    *mode = "Balanced".to_string();
                }
            }

            BatteryModel::get().available_modes.set(available_modes);

            let current_mode = Power::get_current_cpu_governor().await.unwrap();
            let current_mode_value = match current_mode.as_str() {
                "performance\n" => "High",
                "powersave\n" => "Low",
                "conservative\n" => "Balanced",
                _ => "",
            };

            BatteryModel::get()
                .cureent_mode
                .set(current_mode_value.to_string());
        });
    }

    pub fn set_mode(value: &str) {
        let value_map = match value {
            "High" => "performance",
            "Low" => "powersave",
            "Balanced" => "conservative",
            _ => "",
        };
        println!("Set mode value_map: {:?}", value_map);

        RUNTIME.spawn(async {
            Power::set_cpu_governor(value_map.to_string())
                .await
                .unwrap();
        });

        BatteryModel::update();
    }
}
