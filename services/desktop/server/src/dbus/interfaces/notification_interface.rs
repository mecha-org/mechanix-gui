use anyhow::Result;
use command::spawn_command;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::NonZeroU32, path::PathBuf, time::SystemTime};
use tokio::sync::mpsc;
use zbus::{
    interface,
    object_server::SignalContext,
    zvariant::{Signature, Structure, Value},
    Connection,
};

use crate::settings::notifier::NotifierSettings;

#[derive(Debug, Clone)]
pub enum Event {
    New(Notification),
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionId {
    Default,
    Custom(String),
}

impl From<&str> for ActionId {
    fn from(s: &str) -> Self {
        // TODO more actions
        match s {
            "default" => Self::Default,
            s => Self::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hint {
    ActionIcons(bool),
    Category(String),
    DesktopEntry(String),
    Image(Image),
    IconData(Vec<u8>),
    Resident(bool),
    SoundFile(PathBuf),
    SoundName(String),
    SuppressSound(bool),
    Transient(bool),
    Urgency(u8),
    X(i32),
    Y(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub enum Image {
    Name(String),
    File(PathBuf),
    /// RGBA
    Data {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloseReason {
    Expired = 1,
    Dismissed = 2,
    CloseNotification = 3,
    Undefined = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<(ActionId, String)>,
    pub hints: Vec<Hint>,
    pub expire_timeout: i32,
    pub time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct NotificationBusInterface {
    pub event_tx: mpsc::Sender<Event>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationBusInterface {
    async fn close_notification(&self, id: u32) {}

    #[zbus(out_args("name", "vendor", "version", "spec_version"))]
    async fn get_server_information(
        &self,
    ) -> (&'static str, &'static str, &'static str, &'static str) {
        ("mechanix-notifications", "System76", "123", "1.2")
    }

    async fn notify(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        println!("{:?} {:?} {:?} {:?}", app_name, app_icon, summary, body);
        let actions = actions
            .chunks_exact(2)
            .map(|a| (ActionId::from(a[0]), a[1].to_string()))
            .collect();

        let hints = hints
            .into_iter()
            .filter_map(|(k, v)| match k {
                "action-icons" => bool::try_from(v).map(Hint::ActionIcons).ok(),
                "category" => String::try_from(v).map(Hint::Category).ok(),
                "desktop-entry" => String::try_from(v).map(Hint::DesktopEntry).ok(),
                "resident" => bool::try_from(v).map(Hint::Resident).ok(),
                "sound-file" => String::try_from(v)
                    .map(|s| Hint::SoundFile(PathBuf::from(s)))
                    .ok(),
                "sound-name" => String::try_from(v).map(Hint::SoundName).ok(),
                "suppress-sound" => bool::try_from(v).map(Hint::SuppressSound).ok(),
                "transient" => bool::try_from(v).map(Hint::Transient).ok(),
                "x" => i32::try_from(v).map(Hint::X).ok(),
                "y" => i32::try_from(v).map(Hint::Y).ok(),
                "urgency" => u8::try_from(v).map(Hint::Urgency).ok(),
                "image-path" | "image_path" => String::try_from(v).ok().and_then(|s| {
                    if let Some(path) = url::Url::parse(&s).ok().and_then(|u| u.to_file_path().ok())
                    {
                        Some(Hint::Image(Image::File(path)))
                    } else {
                        Some(Hint::Image(Image::Name(s)))
                    }
                }),
                "image-data" | "image_data" | "icon_data" => match v {
                    zbus::zvariant::Value::Structure(v) => match ImageData::try_from(v) {
                        Ok(mut image) => Some({
                            // image = image.into_rgba();
                            Hint::Image(Image::Data {
                                width: image.width,
                                height: image.height,
                                data: image.data,
                            })
                        }),
                        Err(err) => {
                            tracing::warn!("Invalid image data: {}", err);
                            None
                        }
                    },
                    _ => {
                        tracing::warn!("Invalid value for hint: {}", k);
                        None
                    }
                },
                _ => {
                    tracing::warn!("Unknown hint: {}", k);
                    None
                }
            })
            .collect();

        let notification = Notification {
            id: 1,
            app_name: app_name.to_string(),
            app_icon: app_icon.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            actions,
            hints,
            expire_timeout,
            time: SystemTime::now(),
        };
        let _ = self.event_tx.send(Event::New(notification)).await;
        1
    }

    async fn get_capabilities(&self) -> Vec<&'static str> {
        vec![
            "body",
            "icon-static",
            "persistence",
            // TODO support these
            "actions",
            "action-icons",
            "body-markup",
            "body-hyperlinks",
            "sound",
        ]
    }
    #[zbus(signal)]
    async fn action_invoked(
        signal_ctxt: &SignalContext<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(
        signal_ctxt: &SignalContext<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()>;
}

pub struct Notifier {
    pub stack: Vec<Notification>,
    pub is_child_running: bool,
    pub configs: NotifierSettings,
}

impl Notifier {
    pub fn new(configs: NotifierSettings) -> Self {
        Self {
            stack: vec![],
            is_child_running: false,
            configs,
        }
    }

    pub fn add_new(&mut self, n: Notification) -> Result<bool> {
        self.stack.push(n);
        Ok(true)
    }

    pub async fn run(mut self, event_tx: mpsc::Sender<Event>, mut event_rx: mpsc::Receiver<Event>) {
        let events_t = tokio::spawn(async move {
            loop {
                if let Some(event) = event_rx.recv().await {
                    println!("event is {:?}", event);
                    match event {
                        Event::New(n) => {
                            println!(
                                "{:?} {:?}",
                                n,
                                self.configs.run_commands.notification.clone()
                            );
                            let mut args: Vec<String> = vec![];
                            args.push(self.configs.run_commands.notification.clone());
                            args.push(format!("--app-name={:?}", n.app_name.clone()));
                            args.push(format!("--title={:?}", n.summary.clone()));
                            args.push(format!("--description={:?}", n.body.clone()));

                            //Spawn notification shell component
                            let _ = spawn_notification(args.join(" "), event_tx.clone()).await;
                        }
                        Event::Closed => {
                            break;
                        }
                    }
                };
            }
        });
        events_t.await.unwrap();
    }
}

async fn spawn_notification(run_command: String, event_tx: mpsc::Sender<Event>) -> Result<bool> {
    println!("spawn_notification run_command {:?}", run_command);
    let _ = tokio::spawn(async move {
        if !run_command.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(run_command.clone());
            let command_r = spawn_command("sh".to_string(), args);
            if let Err(e) = &command_r {
                println!("Error while spawning child {:?}", e);
                return;
            }

            let mut command = command_r.unwrap();
            let _ = command.wait();
            let _ = event_tx.send(Event::Closed).await;
        }
    })
    .await;
    Ok(true)
}

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub rowstride: i32,
    pub has_alpha: bool,
    pub bits_per_sample: i32,
    pub channels: i32,
    pub data: Vec<u8>,
}

impl<'a> TryFrom<Structure<'a>> for ImageData {
    type Error = zbus::Error;

    fn try_from(value: Structure<'a>) -> zbus::Result<Self> {
        if Ok(value.full_signature()) != Signature::from_static_str("(iiibiiay)").as_ref() {
            return Err(zbus::Error::Failure(format!(
                "Invalid ImageData: invalid signature {}",
                value.full_signature().to_string()
            )));
        }

        let mut fields = value.into_fields();

        if fields.len() != 7 {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: missing fields".to_string(),
            ));
        }

        let data = Vec::<u8>::try_from(fields.remove(6))
            .map_err(|e| zbus::Error::Failure(format!("data: {}", e)))?;
        let channels = i32::try_from(fields.remove(5))
            .map_err(|e| zbus::Error::Failure(format!("channels: {}", e)))?;
        let bits_per_sample = i32::try_from(fields.remove(4))
            .map_err(|e| zbus::Error::Failure(format!("bits_per_sample: {}", e)))?;
        let has_alpha = bool::try_from(fields.remove(3))
            .map_err(|e| zbus::Error::Failure(format!("has_alpha: {}", e)))?;
        let rowstride = i32::try_from(fields.remove(2))
            .map_err(|e| zbus::Error::Failure(format!("rowstride: {}", e)))?;
        let height = i32::try_from(fields.remove(1))
            .map_err(|e| zbus::Error::Failure(format!("height: {}", e)))?;
        let width = i32::try_from(fields.remove(0))
            .map_err(|e| zbus::Error::Failure(format!("width: {}", e)))?;

        if width <= 0 {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: width is not positive".to_string(),
            ));
        }

        if height <= 0 {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: height is not positive".to_string(),
            ));
        }

        if bits_per_sample != 8 {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: bits_per_sample is not 8".to_string(),
            ));
        }

        if has_alpha && channels != 4 {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: has_alpha is true but channels is not 4".to_string(),
            ));
        }

        if (width * height * channels) as usize != data.len() {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: data length does not match width * height * channels"
                    .to_string(),
            ));
        }

        if data.len() != (rowstride * height) as usize {
            return Err(zbus::Error::Failure(
                "Invalid ImageData: data length does not match rowstride * height".to_string(),
            ));
        }

        Ok(Self {
            width: width as u32,
            height: height as u32,
            rowstride,
            has_alpha,
            bits_per_sample,
            channels,
            data,
        })
    }
}
