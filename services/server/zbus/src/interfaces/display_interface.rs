use display::Display;
use zbus::{fdo::Error as ZbusError, interface};

pub struct DisplayBusInterface {}

#[interface(name = "org.mechanix.services.Display")]
impl DisplayBusInterface {
    pub async fn get_brightness(&self) -> Result<u8, ZbusError> {
        let display_path = "/sys/class/backlight/intel_backlight/brightness";
        let display = Display::new(display_path).unwrap();
        let brightness = match display.get_brightness() {
            Ok(brightness) => brightness,
            Err(_) => return Err(ZbusError::Failed("Failed to get brightness".to_string())),
        };
        Ok(brightness)
    }

    pub async fn set_brightness(&self, brightness: u8) -> Result<(), ZbusError> {
        let display_path = "/sys/class/backlight/intel_backlight/brightness";
        let display = Display::new(display_path).unwrap();
        let _ = match display.set_brightness(brightness) {
            Ok(brightness) => brightness,
            Err(_) => return Err(ZbusError::Failed("Failed to set brightness".to_string())),
        };

        Ok(())
    }

    pub async fn set_backlight_on(&self) -> Result<(), ZbusError> {
        let display_path = "/sys/class/backlight/intel_backlight/brightness";
        let display = Display::new(display_path).unwrap();
        let _ = match display.set_backlight_on() {
            Ok(brightness) => brightness,
            Err(_) => return Err(ZbusError::Failed("Failed to set backlight on".to_string())),
        };

        Ok(())
    }

    pub async fn set_backlight_off(&self) -> Result<(), ZbusError> {
        let display_path = "/sys/class/backlight/intel_backlight/brightness";
        let display = Display::new(display_path).unwrap();
        let _ = match display.set_backlight_off() {
            Ok(brightness) => brightness,
            Err(_) => return Err(ZbusError::Failed("Failed to set backlight off".to_string())),
        };

        Ok(())
    }
}
