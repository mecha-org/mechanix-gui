pub const MAX_DEVICE_BRIGHTNESS: u32 = 254;
pub const DEFAULT_MIN_BRIGHTNESS: f32 = 10.;

pub fn u8_to_percent(value: u8, max_u32: u32) -> f32 {
    value as f32 / max_u32 as f32 * 100.0
}
pub fn percent_to_u8(percent: f32, max_u32: u32) -> u8 {
    ((percent / 100.0) * max_u32 as f32).round() as u8
}
