use gpui::SharedString;
use std::time::{SystemTime, UNIX_EPOCH};

// Helper function to format notification preview
pub fn format_notification_name(app_name: &str) -> SharedString {
    if app_name.is_empty() {
        app_name.to_string().into()
    } else {
        // Limit body to reasonable length
        let preview = if app_name.len() > 30 {
            format!("{}...", &app_name[..30])
        } else {
            format!("{}", app_name)
        };
        preview.into()
    }
}

// Helper function to format notification preview
pub fn format_notification_body(app_name: &str, body: &str) -> SharedString {
    if body.is_empty() {
        app_name.to_string().into()
    } else {
        // Limit body to reasonable length
        let body_clean = body.replace('\n', " ");
        let preview = if body_clean.len() > 100 {
            format!("{}: {}...", app_name, &body_clean[..97])
        } else {
            format!("{}: {}", app_name, body_clean)
        };
        preview.into()
    }
}
pub fn format_notification_summary(summary: &str) -> SharedString {
    summary
        .split('(')
        .next()
        .unwrap_or(summary) // fallback (very defensive)
        .trim()
        .to_string()
        .into()
}

/// returns a "time ago" string for a given epoch timestamp
pub fn time_ago(crn_time: u64, ts: u64) -> String {
    let diff = crn_time.saturating_sub(ts);
    match diff {
        0..=59 => "now".to_string(),
        60..=3599 => format!("{}m", diff / 60),
        3600..=86399 => format!("{}hr", diff / 3600),
        _ => format!("{}d", diff / 86_400),
    }
}

/// A cubic bezier function like CSS `cubic-bezier`.
///
/// Builder:
///
/// https://cubic-bezier.com
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> impl Fn(f32) -> f32 {
    move |t: f32| {
        let one_t = 1.0 - t;
        let one_t2 = one_t * one_t;
        let t2 = t * t;
        let t3 = t2 * t;

        // The Bezier curve function for x and y, where x0 = 0, y0 = 0, x3 = 1, y3 = 1
        let _x = 3.0 * x1 * one_t2 * t + 3.0 * x2 * one_t * t2 + t3;
        let y = 3.0 * y1 * one_t2 * t + 3.0 * y2 * one_t * t2 + t3;

        y
    }
}

pub fn should_auto_hide(timeout: i32) -> bool {
    match timeout {
        0 => false, // never auto-hide
        _ => true,  // -1 or any positive value
    }
}

/// helper returning epoch seconds
pub fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn parse_actions(actions: Vec<String>) -> Vec<(String, String)> {
    // Parse into pairs
    let parsed_actions: Vec<(String, String)> = actions
        .chunks(2)
        .filter_map(|c| {
            if c.len() == 2 {
                Some((c[0].clone(), c[1].clone()))
            } else {
                None
            }
        })
        .collect();
    parsed_actions
}
