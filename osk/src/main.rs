use anyhow::Result;
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

use settings::OskSettings;

const CHANNEL_SIZE: usize = 1;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command_index = args.iter().position(|arg| arg == "-cmd");
    match command_index {
        Some(index) => match args.get(index + 1) {
            Some(cmd) => {
                match EchoClient::echo(
                    "org.mechanics.Osk",
                    "/org/mechanics/Osk",
                    "org.mechanics.Osk",
                    cmd,
                )
                .await
                {
                    Ok(r) => {
                        info!("osk echo success");
                    }
                    Err(e) => {
                        error!("osk echo failed {}", e);
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
        .with_env_filter("osk=trace")
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
        Err(_) => OskSettings::default(),
    };

    let _ = init_services(settings).await;
}

async fn init_services(settings: OskSettings) -> Result<bool> {
    info!("init service init");

    let keyboard_settings = settings.bins.keyboard.clone();

    //process
    let (child_process_t, child_process_tx) = init_child_process_handle().await;

    //zbus event manager

    let event_handler_t = init_event_handler(child_process_tx.clone()).await;

    info!("sending message to child process manager to start osk as child process");
    match keyboard_settings.bin {
        Some(bin) => {
            let _ = child_process_tx
                .send(ChildProcessMessage::Spawn {
                    process_name: String::from("osk"),
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
) -> task::JoinHandle<()> {
    let event_handler_t =
        tokio::spawn(async move { ZbusServiceHandle::new().run(child_process_tx).await });

    event_handler_t
}
