use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::Type};

#[proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Manager {
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;

    fn suspend(&self, interactive: bool) -> zbus::Result<()>;

    fn power_off(&self, interactive: bool) -> zbus::Result<()>;

    // fn kill_session(&self, session_id: &str, who: &str, signal_number: i32) -> zbus::Result<()>;
}
