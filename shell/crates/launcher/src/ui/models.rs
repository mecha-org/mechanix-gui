// use gpui::*;
// use tracing::debug;

// pub struct GlobalState {
//     pub wireless_enabled: Entity<bool>,
//     pub wireless_strength: Entity<u8>,
//     pub bluetooth_enabled: Entity<bool>,
//     pub battery_level: Entity<u8>,
// }

// impl Global for GlobalState {}

// pub fn load_global_state(cx: &mut App) {
//     debug!("loading global state");
//     let wireless_enabled: Entity<bool> = cx.new(|_| false);
//     let wireless_strength: Entity<u8> = cx.new(|_| 0);
//     let bluetooth_enabled: Entity<bool> = cx.new(|_| false);
//     let battery_level: Entity<u8> = cx.new(|_| 0);

//     cx.set_global(GlobalState {
//         wireless_enabled,
//         wireless_strength,
//         bluetooth_enabled,
//         battery_level,
//     });
// }
