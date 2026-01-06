pub use settings_drawer::ui::icon::{Icon, IconName};

// this is to use the already implemented Icon finding mechanism in settings_drawer and pull icons for Volume Slider
pub fn icon_for_volume(volume: f32, min: f32, max: f32) -> IconName {
    if volume <= min {
        IconName::VolumeOff
    } else {
        let range = max - min;
        let normalized = (volume - min) / range;
        if normalized <= 0.33 {
            IconName::VolumeLow
        } else if normalized <= 0.66 {
            IconName::VolumeMedium
        } else {
            IconName::VolumeHigh
        }
    }
}
