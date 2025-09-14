use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};
use zbus::{fdo::Error as ZbusError, interface};

#[derive(Type, SerializeDict, DeserializeDict, Debug, Default, Clone)]
#[zvariant(signature = "a{sv}")]
pub struct HapticFeedbackParams {
    duration: u8,
    level: HapticFeedbackLevel,
}

#[derive(Debug, Default, Clone, Type, Serialize, Deserialize)]
enum HapticFeedbackLevel {
    #[default]
    LOW,
    MEDIUM,
    HIGH,
}

#[derive()]
pub struct HapticFeedbackInterface {
    pub path: String,
}

#[interface(name = "org.mechanix.services.HapticFeedback")]
impl HapticFeedbackInterface {
    pub fn trigger_haptic_feedback(
        &self,
        haptic_feedback: HapticFeedbackParams,
    ) -> Result<(), ZbusError> {
        info!("Triggering haptic feedback");
        let mut file = match File::create(&self.path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open haptic feedback file: {}", e);
                return Err(ZbusError::Failed(
                    "haptic feedback file not found".to_string(),
                ));
            }
        };

        //TODO: check once you have actual value
        match file.write_all(haptic_feedback.duration.to_be_bytes().as_slice()) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to write haptic feedback file: {}", err);
                return Err(ZbusError::Failed(
                    "Failed to write haptic feedback file".to_string(),
                ));
            }
        }
        Ok(())
    }
}
