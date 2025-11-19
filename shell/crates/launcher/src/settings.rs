use gpui::*;

#[derive(Debug, Clone)]
pub struct Settings {
    pub window_size: (f32, f32),
}

impl Settings {
    pub fn init() -> Self {
        Self {
            window_size: (540., 620.),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub settings: Entity<Settings>,
}

impl GlobalSettings {
    pub fn init(cx: &mut App) -> Self {
        Self {
            settings: cx.new(|_| Settings::init()),
        }
    }
}

impl Global for GlobalSettings {}

pub fn load_settings(cx: &mut App) -> Result<()> {
    let global = GlobalSettings::init(cx);
    cx.set_global::<GlobalSettings>(global);

    Ok(())
}
