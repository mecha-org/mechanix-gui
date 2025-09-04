use bevy::platform::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SettingsDrawerSettings {
    pub width: f32,
    pub height: f32,
    pub menus: HashMap<String, Vec<String>>,
}

impl Default for SettingsDrawerSettings {
    fn default() -> Self {
        Self {
            width: 100.,
            height: 100.,
            menus: HashMap::from([
                (
                    "sm".to_string(),
                    Vec::from([
                        "airplane_mode".to_string(),
                        "auto_rotation".to_string(),
                        "external_display".to_string(),
                        "screen_record".to_string(),
                        "wifi".to_string(),
                        "bluetooth".to_string(),
                        "camera".to_string(),
                        "battery".to_string(),
                        "terminal".to_string(),
                        "voice_record".to_string(),
                        "calc".to_string(),
                        "theme".to_string(),
                        "brightness".to_string(),
                        "volume".to_string(),
                    ]),
                ),
                // (
                //     "lg".to_string(),
                //     Vec::from([
                //         "airplane_mode".to_string(),
                //         "auto_rotation".to_string(),
                //         "external_display".to_string(),
                //         "screen_record".to_string(),
                //         "wifi".to_string(),
                //         "bluetooth".to_string(),
                //     ]),
                // ),
            ]),
        }
    }
}
