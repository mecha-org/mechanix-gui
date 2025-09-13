use zbus::{fdo::Error as ZbusError, interface, zvariant::Type, SignalContext};

use mechanix_hw_buttons::{HwButton, Key, KeyEvent};

#[derive(Clone, Copy)]
pub struct HwButtonInterface {}

#[interface(name = "org.mechanix.services.HwButton")]
impl HwButtonInterface {
    #[zbus(signal)]
    async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: KeyEvent,
    ) -> Result<(), zbus::Error>;
}

pub async fn hw_buttons_notification_stream(
    hw_button_bus: &HwButtonInterface,
    conn: &zbus::Connection,
    power_button_path: String,
    home_button_path: String,
) -> Result<(), ZbusError> {
    let mut power_button = HwButton::new(power_button_path);
    let mut home_button = HwButton::new(home_button_path);
    loop {
        tokio::select! {
            (key, event) = power_button.poll() => {
                if key == Key::Power {
                    println!("power button event is {:?}", event);
                 let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/Power")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }
            }
            (key, event) = home_button.poll() => {
                if key == Key::Home {println!("home button event is {:?}", event);
                let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/Home")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }
            }
        }
    }
}
