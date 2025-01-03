use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;
use upower::device::DeviceProxy;
use upower::BatteryStatus;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref Battery: BatteryModel = BatteryModel {
        is_running: Context::new(false),
        level: Context::new(0),
        status: Context::new(BatteryStatus::Discharging)
    };
}

#[derive(Model)]
pub struct BatteryModel {
    level: Context<u8>,
    status: Context<BatteryStatus>,
    is_running: Context<bool>,
}

impl BatteryModel {
    pub fn get() -> &'static Self {
        &Battery
    }

    pub fn level() -> u8 {
        *Self::get().level.get()
    }

    pub fn status() -> BatteryStatus {
        Self::get().status.get().clone()
    }

    fn run_battery() {
        RUNTIME.spawn(async {
            let battery_r = upower::get_battery().await;

            if let Err(e) = battery_r {
                println!("Error while getting battery {:?}", e);
                return;
            }

            let battery = battery_r.unwrap();

            if let Ok(percentage) = battery.percentage().await {
                Self::get().level.set(percentage as u8);
            };

            if let Ok(state) = battery.state().await {
                if let Ok(status) = BatteryStatus::try_from(state) {
                    Self::get().status.set(status);
                }
            };

            Self::run_state_stream(battery.clone());
            Self::run_percentage_stream(battery.clone());
        });
    }

    fn run_state_stream(battery: DeviceProxy<'static>) {
        RUNTIME.spawn(async move {
            let mut state_stream = battery.receive_state_changed().await;
            while let Some(msg) = state_stream.next().await {
                if let Ok(state) = msg.get().await {
                    println!("BATTERY STATE {:?}", state);
                    let status = BatteryStatus::try_from(state).unwrap();
                    let level = battery.percentage().await.unwrap() as u8;
                    Self::get().status.set(status);
                    Self::get().level.set(level);
                };
            }
        });
    }

    fn run_percentage_stream(battery: DeviceProxy<'static>) {
        RUNTIME.spawn(async move {
            let mut percentage_stream = battery.receive_percentage_changed().await;
            while let Some(msg) = percentage_stream.next().await {
                if let Ok(percentage) = msg.get().await {
                    let status = BatteryStatus::try_from(battery.state().await.unwrap()).unwrap();
                    let level = percentage as u8;
                    Self::get().status.set(status);
                    Self::get().level.set(level);
                };
            }
        });
    }

    pub fn run() {
        if *BatteryModel::get().is_running.get() {
            return;
        }

        BatteryModel::get().is_running.set(true);
        Self::run_battery();
    }
}
