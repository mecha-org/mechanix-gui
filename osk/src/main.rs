use anyhow::Result;
use event_handler::zbus::ZbusServiceHandle;
use init_tracing_opentelemetry::tracing_subscriber_ext::{
    build_logger_text, build_loglevel_filter_layer, build_otel_layer,
};
use process::handler::{ChildProcessHandle, ChildProcessMessage};
use sentry_tracing::{self, EventFilter};
use std::error::Error;
use tokio::{sync::mpsc, task};
use tracing::info;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
mod event_handler;
mod process;

const CHANNEL_SIZE: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let subscriber = tracing_subscriber::registry()
        .with(sentry_tracing::layer().event_filter(|_| EventFilter::Ignore))
        .with(build_loglevel_filter_layer()) //temp for terminal log
        .with(build_logger_text()) //temp for terminal log
        .with(build_otel_layer().unwrap()); // trace collection layer
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!(
        //sample log
        task = "tracing_setup",
        result = "success",
        "tracing set up",
    );
    let _ = init_services().await;
    Ok(())
}

async fn init_services() -> Result<bool> {
    info!("init service init");

    //process
    let (child_process_t, child_process_tx) = init_child_process_handle().await;

    //zbus event manager

    let event_handler_t = init_event_handler(child_process_tx.clone()).await;

    info!("sending message to osk as chilf process");
    let _ = child_process_tx
        .send(ChildProcessMessage::Spawn {
            //process_name: String::from("osk"),
            process_name: String::from("osk"),
            command: String::from("gnome-calculator"),
            args: vec![],
        })
        .await;

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
