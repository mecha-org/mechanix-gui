use std::{collections::HashMap, process::Stdio};

use anyhow::Result;
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    process::{Child, Command},
    select,
    signal::unix::SignalKind,
    sync::{broadcast, mpsc},
};
use tonic::async_trait;
use tracing::{error, info};

#[derive(Debug)]
pub enum DaemonMessage {
    Spawn {
        process_name: String,
        command: String,
        args: Vec<String>,
    },
    Signal {
        process_name: String,
        code: i32,
    },
}

pub struct DaemonState {
    processes: HashMap<String, Child>,
}

pub struct DaemonHandle {
    status: ServiceStatus,
    state: DaemonState,
}

impl DaemonHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            state: DaemonState {
                processes: HashMap::new(),
            },
        }
    }

    pub async fn run(&mut self, mut message_rx: mpsc::Receiver<DaemonMessage>) {
        info!("DaemonHandle run called");
        // start the service
        //let _ = &self.start().await;

        loop {
            select! {
                msg = message_rx.recv() => {
                    info!("msg is {:?}", msg);

                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        DaemonMessage::Spawn { process_name, command, args } => {
                            println!("DaemonHandle run received Spawn {} {} {:?}", process_name, command, args);
                            let _ = self.spawn(&process_name, &command, &[]).await;
                         }
                        DaemonMessage::Signal { process_name, code } => {
                            println!("DaemonHandle run received Signal {} {:?}", process_name, code);
                            let _ = self.signal(&process_name, code).await;
                        }
                    };
                }
            }
        }
    }
}

impl DaemonHandle {
    pub async fn spawn(&mut self, process_name: &str, command: &str, args: &[&str]) {
        info!("DaemonHandle spawn {} {:?}", process_name, args);

        let child = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start child process");

        info!("child process started with id {:?}", child.id());

        self.state
            .processes
            .insert(String::from(process_name), child);
    }

    pub async fn signal(&mut self, process_name: &str, process_signal: i32) {
        info!("DaemonHandle signal {} {:?}", process_name, process_signal);
    }
}

#[async_trait]
impl ServiceHandler for DaemonHandle {
    async fn start(&mut self) -> Result<bool> {
        Ok(true)
    }

    async fn stop(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STOPPED;
        Ok(true)
    }

    fn get_status(&self) -> anyhow::Result<ServiceStatus> {
        Ok(self.status)
    }

    fn is_stopped(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STOPPED)
    }

    fn is_started(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STARTED)
    }
}
