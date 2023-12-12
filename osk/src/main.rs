use anyhow::Result;
use process::handler::ChildProcessHandle;
use tokio::{sync::oneshot, task};

mod event_handler;
mod process;
pub enum ChildProcessMessage {
    Show,
    Hide,
    Toggle,
}

const CHANNEL_SIZE: usize = 1;

fn main() {
    println!("Hello, world!");
    init_services();
}

fn init_services() -> Result<bool> {
    //process
    let child_process_handle = ChildProcessHandle::new();

    //zbus event manager

    Ok(true)
}

async fn init_child_process_handle() -> (task::JoinHandle<()>, oneshot::Sender<ChildProcessMessage>)
{
    let (child_process_tx, child_process_rx) = oneshot::channel();

    let child_process_t =
        tokio::spawn(async move { ChildProcessHandle::new().run(child_process_rx).await });

    (child_process_t, child_process_tx)
}
