use anyhow::{bail, Result};
use echo_client::EchoClient;
use handlers::{
    rotation::handler::{RotationHandler, RotationHandlerMessage},
    transform::handler::{WlrootsHandler, WlrootsHandlerMessage},
    zbus::zbus::ZbusServiceHandle,
};
use std::error::Error;
use tokio::{
    sync::{broadcast, mpsc},
    task,
};
use tracing::{error, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
mod backends;
mod errors;
mod handlers;
mod settings;

use settings::RotationSettings;

const CHANNEL_SIZE: usize = 1;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("rotation=trace")
        .with_thread_names(true)
        .init();
    tracing::info!(
        //sample log
        task = "tracing_setup",
        result = "success",
        "tracing set up",
    );
    let args: Vec<String> = std::env::args().collect();
    info!("args are {:?}", args);
    let command_index = args.iter().position(|arg| arg == "-cmd");
    match command_index {
        Some(index) => match args.get(index + 1) {
            Some(cmd) => {
                match EchoClient::echo(
                    "org.mechanics.Rotation",
                    "/org/mechanics/Rotation",
                    "org.mechanics.Rotation",
                    cmd,
                    (
                        args.get(index + 2).unwrap_or(&String::from("")),
                        args.get(index + 3).unwrap_or(&String::from("")).as_str() == "true",
                    ),
                )
                .await
                {
                    Ok(r) => {
                        info!("Rotation echo success");
                    }
                    Err(e) => {
                        error!("Rotation echo failed {}", e);
                    }
                };
                return;
            }
            None => {
                return;
            }
        },
        None => (),
    };

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => RotationSettings::default(),
    };

    let _ = init_services(settings).await;
}

async fn init_services(settings: RotationSettings) -> Result<bool> {
    info!("init service init");

    //init wlroots hander, this will keep watching for events from rotation handler

    let (wlroots_hanlder_th, wlroots_handler_tx) = init_wlroots_handler().await;

    //init rotation handler, this will keep checking accelerometer, and will send events to wlroots
    let (rotation_handler_th, rotation_handler_tx) =
        init_rotation_handler(wlroots_handler_tx, settings.clone()).await;

    //init zbus handler, this will run on cmd input, and will update rotation handler settings
    //if enabled is false then rotation handler will turn off sending events to wlroots
    let (zbus_handler_th) = init_zbus_handler(rotation_handler_tx).await;

    wlroots_hanlder_th.await.unwrap();

    rotation_handler_th.await.unwrap();

    zbus_handler_th.await.unwrap();

    Ok(true)
}

async fn init_wlroots_handler() -> (task::JoinHandle<()>, mpsc::Sender<WlrootsHandlerMessage>) {
    let (tx, rx) = mpsc::channel(128);

    let th = tokio::spawn(async move { WlrootsHandler::new().run(rx).await });

    (th, tx)
}

async fn init_rotation_handler(
    wlroots_handler_tx: mpsc::Sender<WlrootsHandlerMessage>,
    settings: RotationSettings,
) -> (
    task::JoinHandle<()>,
    broadcast::Sender<RotationHandlerMessage>,
) {
    let (tx, rx) = broadcast::channel(128);

    let th = tokio::spawn(async move {
        RotationHandler::new(wlroots_handler_tx)
            .run(rx, settings)
            .await
    });

    (th, tx)
}

async fn init_zbus_handler(
    rotation_handler_tx: broadcast::Sender<RotationHandlerMessage>,
) -> task::JoinHandle<()> {
    let event_handler_t =
        tokio::spawn(async move { ZbusServiceHandle::new(rotation_handler_tx).run().await });

    event_handler_t
}
