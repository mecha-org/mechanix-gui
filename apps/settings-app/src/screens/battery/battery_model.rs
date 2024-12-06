use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref BATTERY_MODEL: BatteryModel = BatteryModel {
        battery_percentage: Context::new(0.0)
    };
}

#[derive(Model)]
pub struct BatteryModel {
    pub battery_percentage: Context<f64>,
}

impl BatteryModel {
    pub fn get() -> &'static Self {
        &BATTERY_MODEL
    }

    pub fn update() {
        RUNTIME.spawn(async {
            let battery = upower::get_battery().await.unwrap();
            let percentage = battery.percentage().await.unwrap();
            println!("Battery percentage: {}", percentage);
            BatteryModel::get().battery_percentage.set(percentage);
        });
    }
}
