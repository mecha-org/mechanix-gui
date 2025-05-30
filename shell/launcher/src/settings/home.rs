use bevy::platform::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Widget {
    name: String,
    column_grid_span: u16,
    row_grid_span: u16,

}

#[derive(Debug, Clone)]
pub struct HomeScreenSettings {
    pub width: f32,
    pub height: f32,
    pub pinned_apps: HashMap<String, Vec<String>>,
    pub widgets: HashMap<String, Vec<Widget>>,
}

impl Default for HomeScreenSettings {
    fn default() -> Self {
        Self {
            width: 100.,
            height: 100.,
            pinned_apps: HashMap::from([
                (
                    "sm".to_string(),
                    Vec::from([
                        "App 1".to_string(),
                        "App 2".to_string(),
                        "App 3".to_string(),
                        "App 4".to_string(),
                        "App 5".to_string(),
                        "App 6".to_string(),
                        "App 7".to_string(),
                        "App 8".to_string(),
                        "App 9".to_string(),
                        "App 10".to_string(),
                        "App 11".to_string(),
                        "App 12".to_string(),
                        "App 13".to_string(),
                        "App 14".to_string(),
                        "App 15".to_string(),
                        "App 16".to_string(),
                    ]),
                ),
            ]),
            // widgets: HashMap::from([
            //     (
            //         "md".to_string(),
            //         Vec::from([
            //             "CPU Usage".to_string(),
            //             "Watch".to_string(),
            //         ]),
            //     ),

            // ]),
            widgets: HashMap::from([
                (
                    "md".to_string(),
                    Vec::from([
                        Widget {
                            name: "CPU Usage".to_string(),
                            column_grid_span: 2,
                            row_grid_span: 2,
                        },
                        
                    ]),
                ),
            ]),
        }
    }
}
