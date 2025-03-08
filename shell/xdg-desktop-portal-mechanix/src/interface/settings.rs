use std::collections::HashMap;
use zbus::zvariant::{DeserializeDict, OwnedValue, SerializeDict, Type, Value};
use zbus::{fdo, interface};

const APPEARANCE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME: &str = "color-scheme";
const ACCENT_COLOR: &str = "accent-color";
const CONTRAST: &str = "contrast";

#[derive(DeserializeDict, SerializeDict, Clone, Copy, PartialEq, Type, OwnedValue, Value)]
pub struct AccentColor {
    red: f64,
    green: f64,
    blue: f64,
}

impl AccentColor {
    pub fn new(rgb: [f64; 3]) -> Self {
        Self {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
        }
    }
}

#[derive(Debug)]
pub struct Settings {}

#[interface(name = "org.freedesktop.impl.portal.Settings")]
impl Settings {
    async fn read_one(&self, namespace: String, key: String) -> fdo::Result<OwnedValue> {
        if namespace != APPEARANCE {
            return Err(zbus::fdo::Error::Failed("No such namespace".to_string()));
        }
        match key.as_str() {
            COLOR_SCHEME => Ok(OwnedValue::from(0)), // Default color scheme
            ACCENT_COLOR => Ok(AccentColor::new([0.0, 0.0, 0.0]).try_into().unwrap()), // Default accent color
            CONTRAST => Ok(OwnedValue::from(0)), // Default contrast
            _ => Err(zbus::fdo::Error::Failed("No such key".to_string())),
        }
    }

    async fn read_all(
        &self,
        namespaces: Vec<&str>,
    ) -> fdo::Result<HashMap<String, HashMap<String, OwnedValue>>> {
        if !namespaces.contains(&APPEARANCE) {
            return Err(zbus::fdo::Error::Failed("No such namespace".to_string()));
        }
        let mut output_setting = HashMap::<String, OwnedValue>::new();
        output_setting.insert(COLOR_SCHEME.to_string(), 0.into()); // Default color scheme
        output_setting.insert(
            ACCENT_COLOR.to_string(),
            OwnedValue::try_from(AccentColor::new([0.0, 0.0, 0.0])).unwrap(), // Default accent color
        );
        output_setting.insert(CONTRAST.to_string(), 0.into()); // Default contrast
        let output = HashMap::<String, HashMap<String, OwnedValue>>::from_iter([(
            APPEARANCE.to_string(),
            output_setting,
        )]);
        Ok(output)
    }
}
