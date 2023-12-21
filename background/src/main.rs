use anyhow::{bail, Result};
use daemon::handler::{DaemonHandle, DaemonMessage};
use echo_client::EchoClient;
use handlers::{
    background::handler::{BackgroundHandler, BackgroundHandlerMessage},
    zbus::zbus::ZbusServiceHandle,
};
use serde::Serialize;
use std::{error::Error, process::Command};
use tokio::{
    sync::{self, broadcast, mpsc},
    task,
};
use tracing::{error, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
mod errors;
mod handlers;
mod settings;

use settings::BackgroundSettings;

use crate::errors::{BackgroundError, BackgroundErrorCodes};

const CHANNEL_SIZE: usize = 1;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("background=trace")
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
                    "org.mechanics.Background",
                    "/org/mechanics/Background",
                    "org.mechanics.Background",
                    cmd,
                    (
                        args.get(index + 2).unwrap_or(&String::from("")),
                        args.get(index + 3).unwrap_or(&String::from("")),
                        args.get(index + 4).unwrap_or(&String::from("")).as_str() == "true",
                    ),
                )
                .await
                {
                    Ok(r) => {
                        info!("background echo success");
                    }
                    Err(e) => {
                        error!("background echo failed {}", e);
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

    println!("Hello, world!");

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => BackgroundSettings::default(),
    };

    let _ = init_services(settings).await;
}

async fn init_services(settings: BackgroundSettings) -> Result<bool> {
    info!("init service init");

    //background handler
    let (background_handler_th, background_handler_tx) =
        init_background_handler(settings.clone()).await;

    // background daemon handler
    let (daemon_handler_th, daemon_handler_tx) =
        init_daemon_handler(background_handler_tx.clone()).await;

    //zbus event handler
    let zbus_handler_th = init_zbus_handler(background_handler_tx.clone()).await;

    info!("sending message to child process manager to start bin as child process");
    match &settings.provider.bin {
        Some(bin) => {
            let _ = daemon_handler_tx
                .send(DaemonMessage::Spawn {
                    process_name: String::from(&settings.provider.kind.unwrap_or_default()),
                    command: String::from(bin),
                    args: vec![],
                })
                .await;
        }
        None => (),
    }

    background_handler_th.await.unwrap();

    daemon_handler_th.await.unwrap();

    zbus_handler_th.await.unwrap();

    Ok(true)
}

async fn init_background_handler(
    background_settings: BackgroundSettings,
) -> (
    task::JoinHandle<()>,
    broadcast::Sender<BackgroundHandlerMessage>,
) {
    let (tx, rx) = broadcast::channel(128);

    let th = tokio::spawn(async move {
        tokio::spawn(async move { BackgroundHandler::new().run(rx, background_settings).await });
    });

    (th, tx)
}

async fn init_daemon_handler(
    background_handler_tx: broadcast::Sender<BackgroundHandlerMessage>,
) -> (task::JoinHandle<()>, mpsc::Sender<DaemonMessage>) {
    let (tx, rx) = mpsc::channel(32);

    let th = tokio::spawn(async move { DaemonHandle::new().run(rx).await });

    (th, tx)
}

async fn init_zbus_handler(
    background_handler_tx: broadcast::Sender<BackgroundHandlerMessage>,
) -> task::JoinHandle<()> {
    let event_handler_t =
        tokio::spawn(async move { ZbusServiceHandle::new(background_handler_tx).run().await });

    event_handler_t
}
