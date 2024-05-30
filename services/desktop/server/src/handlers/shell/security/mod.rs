use anyhow::Result;
use logind::session_lock;
use zbus::{Connection, Proxy};
// use mechanix_desktop_dbus_client::security::Security;

pub struct SecurityHandler {}
impl SecurityHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(mut self) {
        let pin_enabled_r = is_pin_enabled().await;
        if let Err(e) = pin_enabled_r {
            println!("Error while checking pin enabled {:?}", e);
            return;
        }

        let pin_enabled = pin_enabled_r.unwrap();
        println!("pin_enabled {:?}", pin_enabled);
        if pin_enabled {
            let _ = session_lock().await;
        }
    }
}

async fn is_pin_enabled() -> Result<bool> {
    let connection = Connection::session().await?;
    let p = Proxy::new(
        &connection,
        "org.mechanix.services.Security",
        "/org/mechanix/services/Security",
        "org.mechanix.services.Security",
    )
    .await?;
    let pin_enabled: bool = p.call("IsPinLockEnabled", &()).await?;
    Ok(pin_enabled)
}
