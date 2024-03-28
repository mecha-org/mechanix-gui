use crate::errors::{DisplayError, DisplayErrorCodes};
use anyhow::{bail, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};
use tracing::{error as trace_error, info, instrument, trace, warn};
#[derive(Debug, Default)]
pub struct Display {
    pub path: String,
}

impl Display {
    pub fn new(path: &str) -> Result<Self, DisplayError> {
        // Check if the path is valid
        if !Path::new(path).exists() {
            return Err(DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessPathError,
                "Invalid path".to_string(),
            ));
        }

        trace!(task = "display_ctrl instance", "init");
        Ok(Display {
            path: String::from(path),
        })
    }

    #[instrument(skip(self))]
    pub fn set_brightness(&self, brightness: u8) -> Result<()> {
        trace!(task = "set_display_brightness", "init");
        // Check if the brightness value is valid
        if brightness > 244 {
            warn!(task = "set_display_brightness", "invalid brightness value");
            bail!(DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessValueError,
                "invalid brightness value".to_string(),
            ));
        }

        let mut file = File::create(&self.path).with_context(|| {
            trace_error!(
                task = "set_display_brightness",
                "failed to open brightness file"
            );
            DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessPathError,
                "failed to open brightness file".to_string(),
            )
        })?;

        // Try to write the brightness value to the file or return an error
        if let Err(e) = write!(file, "{}", brightness) {
            trace_error!(
                task = "set_display_brightness",
                "unable to write brightness value: {}",
                e
            );
            bail!(DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }

        info!(
            task = "set_display_brightness",
            "set brightness to {}", brightness
        );

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_brightness(&self) -> Result<u8> {
        trace!(task = "get_display_brightness", "init");
        let file = File::open(&self.path).with_context(|| {
            trace_error!(
                task = "get_display_brightness",
                "failed to open brightness file"
            );
            DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessPathError,
                "Failed to open brightness file".to_string(),
            )
        })?;

        let buffer = BufReader::new(file);
        let buffer_value = buffer.lines().next().with_context(|| {
            trace_error!(
                task = "get_display_brightness",
                "failed to read brightness value"
            );
            DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessValueError,
                "Failed to read brightness value".to_string(),
            )
        })?;

        let value = buffer_value?
            .trim() // Use the ? operator to extract the String and propagate errors if any.
            .parse::<u8>()
            .with_context(|| {
                trace_error!(
                    task = "get_display_brightness",
                    "failed to parse brightness value"
                );
                DisplayError::new(
                    DisplayErrorCodes::InvalidBrightnessValueError,
                    "Failed to parse brightness value".to_string(),
                )
            })?;

        info!(
            task = "get_display_brightness",
            "brightness value: {}", value
        );

        Ok(value)
    }

    #[instrument(skip(self))]
    //set backlight on
    pub fn set_backlight_on(&self) -> Result<()> {
        trace!(task = "set_backlight_on", "init");
        let mut file = File::create(&self.path).with_context(|| {
            trace_error!(task = "set_backlight_on", "failed to open brightness file");
            DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessPathError,
                "failed to open brightness file".to_string(),
            )
        })?;
        let brightness: u8 = 255;
        // Try to write the brightness value to the file or return an error

        if let Err(e) = write!(file, "{}", brightness) {
            trace_error!(
                task = "set_display_brightness",
                "unable to write brightness value: {}",
                e
            );
            bail!(DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }

        info!(task = "set_backlight_on", "set backlight on");

        Ok(())
    }

    #[instrument(skip(self))]
    //set backlight off
    pub fn set_backlight_off(&self) -> Result<()> {
        trace!(task = "set_backlight_off", "init");
        let mut file = File::create(&self.path).with_context(|| {
            trace_error!(task = "set_backlight_off", "failed to open brightness file");
            DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessPathError,
                "failed to open brightness file".to_string(),
            )
        })?;
        let brightness: u8 = 0;
        // Try to write the brightness value to the file or return an error

        if let Err(e) = write!(file, "{}", brightness) {
            trace_error!(
                task = "set_display_brightness",
                "unable to write brightness value: {}",
                e
            );
            bail!(DisplayError::new(
                DisplayErrorCodes::InvalidBrightnessValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }

        info!(task = "set_backlight_off", "set backlight off");

        Ok(())
    }
}
