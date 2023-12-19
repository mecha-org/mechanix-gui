use anyhow::{bail, Result};
use echo_client::EchoClient;
use event_handler::zbus::ZbusServiceHandle;
use process::handler::{ChildProcessHandle, ChildProcessMessage};
use std::error::Error;
use tokio::{sync::mpsc, task};
use tracing::{error, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
mod errors;
mod event_handler;
mod process;
mod settings;

use settings::{BackgroundDaemon, BackgroundSettings};

const CHANNEL_SIZE: usize = 1;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command_index = args.iter().position(|arg| arg == "-cmd");
    match command_index {
        Some(index) => match args.get(index + 1) {
            Some(cmd) => {
                match EchoClient::echo(
                    "org.mechanics.Background",
                    "/org/mechanics/Background",
                    "org.mechanics.Background",
                    cmd,
                    (args.get(index + 2).unwrap_or(&String::from(""))),
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
            None => (),
        },
        None => (),
    };

    println!("Hello, world!");
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

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => BackgroundSettings::default(),
    };

    let _ = init_services(settings).await;
}

async fn init_services(settings: BackgroundSettings) -> Result<bool> {
    info!("init service init");

    let default_background_daemon_op = settings
        .bins
        .background_daemons
        .into_iter()
        .find(|daemon| daemon.is_default);

    let background_daemon = match default_background_daemon_op {
        Some(default_background_daemon) => default_background_daemon,
        None => {
            bail!("Default background daemon not found");
        }
    };

    //process
    let (child_process_t, child_process_tx) = init_child_process_handle().await;

    //zbus event manager

    let event_handler_t =
        init_event_handler(child_process_tx.clone(), background_daemon.clone()).await;

    info!("sending message to child process manager to start bin as child process");
    match background_daemon.bin {
        Some(bin) => {
            let _ = child_process_tx
                .send(ChildProcessMessage::Spawn {
                    process_name: String::from(background_daemon.kind.unwrap_or_default()),
                    command: bin,
                    args: vec![],
                })
                .await;
        }
        None => (),
    }

    child_process_t.await.unwrap();

    event_handler_t.await.unwrap();

    Ok(true)
}

async fn init_child_process_handle() -> (task::JoinHandle<()>, mpsc::Sender<ChildProcessMessage>) {
    let (child_process_tx, child_process_rx) = mpsc::channel(32);

    let child_process_t =
        tokio::spawn(async move { ChildProcessHandle::new().run(child_process_rx).await });

    (child_process_t, child_process_tx)
}

async fn init_event_handler(
    child_process_tx: mpsc::Sender<ChildProcessMessage>,
    background_daemon: BackgroundDaemon,
) -> task::JoinHandle<()> {
    let event_handler_t = tokio::spawn(async move {
        ZbusServiceHandle::new(background_daemon)
            .run(child_process_tx)
            .await
    });

    event_handler_t
}
