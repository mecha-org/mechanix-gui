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
        dbg!("Read One Method called, {} {}", &namespace, &key);
        if namespace != APPEARANCE {
            return Err(zbus::fdo::Error::Failed("No such namespace".to_string()));
        }
        match key.as_str() {
            COLOR_SCHEME => Ok(OwnedValue::from(1)),
            ACCENT_COLOR => Ok(AccentColor::new([1.1, 1.0, 1.0]).try_into().unwrap()),
            CONTRAST => Ok(OwnedValue::from(0)),
            _ => Err(zbus::fdo::Error::Failed("No such key".to_string())),
        }
    }

    async fn read_all(
        &self,
        namespaces: Vec<&str>,
    ) -> fdo::Result<HashMap<String, HashMap<String, OwnedValue>>> {
        dbg!("Read all Method called, {:?}", &namespaces);
        if !namespaces.contains(&APPEARANCE) {
            return Err(zbus::fdo::Error::Failed("No such namespace".to_string()));
        }
        let mut output_setting = HashMap::<String, OwnedValue>::new();
        output_setting.insert(COLOR_SCHEME.to_string(), 0.into());
        output_setting.insert(
            ACCENT_COLOR.to_string(),
            OwnedValue::try_from(AccentColor::new([1.0, 1.0, 1.0])).unwrap(),
        );
        output_setting.insert(CONTRAST.to_string(), 0.into());
        let output = HashMap::<String, HashMap<String, OwnedValue>>::from_iter([(
            APPEARANCE.to_string(),
            output_setting,
        )]);
        Ok(output)
    }
}
