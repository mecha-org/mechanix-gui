use chrono::{DateTime, Local, Timelike};
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::{interval_at, Instant};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref Clock: ClockModel = ClockModel {
        date_time: Context::new(Local::now()),
        is_streaming: Context::new(false)
    };
}

#[derive(Model)]
pub struct ClockModel {
    date_time: Context<DateTime<Local>>,
    is_streaming: Context<bool>,
}

impl ClockModel {
    pub fn get() -> &'static Self {
        &Clock
    }

    pub fn date(fmt: &String) -> String {
        format!("{}", Self::get().date_time.get().format(&fmt))
    }

    pub fn time(fmt: &String) -> String {
        format!("{}", Self::get().date_time.get().format(&fmt))
    }

    fn stream_clock() {
        RUNTIME.spawn(async {
            let remaining_seconds = 60 - chrono::Local::now().second() as u64;
            let start = Instant::now() + Duration::from_secs(remaining_seconds);
            let mut interval = interval_at(start, Duration::from_secs(60));

            loop {
                interval.tick().await;
                let date_time = Local::now();
                Self::get().date_time.set(date_time);
            }
        });
    }

    pub fn start_streaming() {
        if *ClockModel::get().is_streaming.get() {
            return;
        }

        ClockModel::get().is_streaming.set(true);
        Self::stream_clock();
    }
}
