use std::process::Command;

use wayland_client::protocol::wl_output::Transform;

use super::{DisplayManager, Orientation};

pub struct XorgBackend {
    touchscreens: Vec<String>,
    target_display: String,
}

impl XorgBackend {
    pub fn new(display: &str, touchscreens: Vec<String>) -> Self {
        XorgBackend {
            target_display: display.into(),
            touchscreens,
        }
    }
}

impl DisplayManager for XorgBackend {
    fn change_rotation_state(&mut self, new_state: &Orientation) {
        Command::new("xrandr")
            .arg("-o")
            .arg(new_state.x_state)
            .spawn()
            .expect("Xrandr rotate command failed to start")
            .wait()
            .expect("Xrandr rotate command wait failed");

        // Support Touchscreen and Styli on some 2-in-1 devices
        for touchscreen in &self.touchscreens {
            Command::new("xinput")
                .arg("set-prop")
                .arg(touchscreen)
                .arg("Coordinate Transformation Matrix")
                .args(new_state.matrix)
                .spawn()
                .expect("Xinput rotate command failed to start")
                .wait()
                .expect("Xinput rotate command wait failed");
        }
    }

    fn get_rotation_state(&mut self) -> Result<Transform, String> {
        let raw_rotation_state = String::from_utf8(
            Command::new("xrandr")
                .output()
                .expect("Xrandr get outputs command failed to start")
                .stdout,
        )
        .unwrap();
        let xrandr_output_pattern = regex::Regex::new(format!(
                r"^{} connected .+? .+? (normal |inverted |left |right )?\(normal left inverted right x axis y axis\) .+$",
                regex::escape(&self.target_display),
            ).as_str()).unwrap();
        for xrandr_output_line in raw_rotation_state.split('\n') {
            if !xrandr_output_pattern.is_match(xrandr_output_line) {
                continue;
            }

            let xrandr_output_captures =
                xrandr_output_pattern.captures(xrandr_output_line).unwrap();
            if let Some(transform) = xrandr_output_captures.get(1) {
                return Ok(match transform.as_str() {
                    "inverted" => Transform::_180,
                    "right" => Transform::_270,
                    "left" => Transform::_90,
                    _ => Transform::Normal,
                });
            } else {
                return Ok(Transform::Normal);
            }
        }

        Err(format!(
            "Unable to determine rotation state: display {} not found in xrandr output",
            self.target_display
        ))
    }
}
