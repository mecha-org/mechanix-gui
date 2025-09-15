use crate::hw_button_client::{HwButton, NotificationArgs};
use futures::executor::ThreadPool;
use futures::{select, StreamExt};
use log::error;
use mechanix_hw_buttons::KeyEvent;
use std::sync::{mpsc, LazyLock};

static THREAD_POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPool::new().expect("Failed to build pool"));
#[derive(Clone)]
pub struct DesktopService {}

#[derive(Debug, Clone)]
pub enum BluetoothEvent {
    DeviceAdded,
}
impl DesktopService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn stream_hw_button_interrupt_event(&self) -> mpsc::Receiver<KeyEvent> {
        let (sender, receiver) = mpsc::channel();
        let mut power_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Power".to_string())
                .await
                .unwrap();
        let mut home_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Home".to_string())
                .await
                .unwrap();

        THREAD_POOL.spawn_ok(async move {
            loop {
                select! {
                    home_button_event = home_stream.next() => {
                        if let Some(event) = home_button_event {
                            if let Err(e) = sender.send(event.args().unwrap().event) {
                                error!("failed to send home interrupt to receiver: {}", e);
                                continue;
                            }
                        }
                    }
                    power_button_event = power_stream.next() => {
                        if let Some(event) = power_button_event {
                            if let Err(e) = sender.send(event.args().unwrap().event) {
                                error!("failed to send power interrupt to receiver: {}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        });
        receiver
    }

    /// Streams home button interrupts.
    ///
    /// This function will spawn a new task on the thread pool that will
    /// listen for home button interrupts and send them to the provided
    /// receiver.
    ///
    /// # Errors
    ///
    /// If the task fails to get the notification stream or send an event
    /// to the receiver, an error will be logged.
    ///
    /// # Examples
    ///
    ///
    pub async fn stream_home_interrupt(&self) -> mpsc::Receiver<KeyEvent> {
        let (sender, receiver) = mpsc::channel();

        THREAD_POOL.spawn_ok(async move {
            match HwButton::get_notification_stream(
                "/org/mechanix/services/HwButton/Home".to_string(),
            )
            .await
            {
                Ok(mut stream) => {
                    while let Some(signal) = stream.next().await {
                        let key_event = match &signal.args() {
                            Ok(args) => args.event, // Clone the args to own the data
                            Err(e) => {
                                error!("failed to get args: {}", e);
                                continue;
                            }
                        };
                        println!("Event: {:?}", key_event);
                        if let Err(e) = sender.send(key_event) {
                            error!("failed to send home interrupt to receiver: {}", e);
                            continue;
                        }
                    }
                }
                Err(_) => {}
            }
        });
        receiver
    }
}
