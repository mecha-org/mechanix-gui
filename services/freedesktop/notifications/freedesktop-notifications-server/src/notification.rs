use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Notification {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, zbus::zvariant::OwnedValue>,
    pub expire_timeout: i32,
}

impl Notification {
    /// Pretty prints the notification with formatting
    pub fn print(&self, id: u32) {
        println!("╭─ Notification ──────────────────────────────────");
        println!("│ ID: {}", id);
        println!("│ App: {}", if self.app_name.is_empty() { "<unknown>" } else { &self.app_name });
        println!("│ Summary: {}", self.summary);
        println!("│ Replace ID: {}", self.replaces_id);
        if !self.body.is_empty() {
            println!("│ Body: {}", self.body);
        }

        if !self.app_icon.is_empty() {
            println!("│ Icon: {}", self.app_icon);
        }

        if !self.actions.is_empty() {
            println!("│ Actions: {:?}", self.actions);
        }

        if self.expire_timeout > 0 {
            println!("│ Expires: {}ms", self.expire_timeout);
        } else if self.expire_timeout == 0 {
            println!("│ Expires: Never");
        } else {
            println!("│ Expires: Server default");
        }

        if !self.hints.is_empty() {
            println!("│ Hints: {} entries", self.hints.len());
            for (key, value) in &self.hints {
                println!("│   {}: {:?}", key, value);
            }
        }

        println!("╰─────────────────────────────────────────────────");
    }
}
