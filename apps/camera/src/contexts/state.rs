use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
lazy_static! {
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    pub static ref STATE: State = State {
        settings_open: Context::new(false),
    };
}

#[derive(Model)]
pub struct State {
    settings_open: Context<bool>,
}

impl State {
    pub fn get() -> &'static Self {
        &STATE
    }

    pub fn set_settings_state(state: bool) {
        STATE.settings_open.set(state);
    }

    pub fn get_settings_state() -> bool {
        *STATE.settings_open.get()
    }
}
