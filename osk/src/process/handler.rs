use std::{collections::HashMap, process::Stdio};

use anyhow::Result;
use libc::kill;
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    process::{Child, Command},
    select,
    signal::unix::SignalKind,
    sync::mpsc,
};
use tonic::async_trait;
use tracing::{error, info};

#[derive(Debug)]
pub enum ChildProcessMessage {
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

pub struct ChildProcessesState {
    child_processes: HashMap<String, Child>,
}

pub struct ChildProcessHandle {
    status: ServiceStatus,
    state: ChildProcessesState,
}

impl ChildProcessHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            state: ChildProcessesState {
                child_processes: HashMap::new(),
            },
        }
    }

    pub async fn run(&mut self, mut message_rx: mpsc::Receiver<ChildProcessMessage>) {
        info!("ChildProcessHandle run called");
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
                        ChildProcessMessage::Spawn { process_name, command, args } => {
                            println!("ChildProcessHandle run received Spawn {} {} {:?}", process_name, command, args);
                            let _ = self.spawn(&process_name, &command, &[]).await;
                         }
                        ChildProcessMessage::Signal { process_name, code } => {
                            println!("ChildProcessHandle run received Signal {} {:?}", process_name, code);
                            let _ = self.signal(&process_name, code).await;
                        }
                    };
                }
            }
        }
    }
}

impl ChildProcessHandle {
    pub async fn spawn(&mut self, process_name: &str, command: &str, args: &[&str]) {
        info!("ChildProcessHandle spawn {} {:?}", process_name, args);

        let child = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start child process");

        info!("child process started with id {:?}", child.id());

        self.state
            .child_processes
            .insert(String::from(process_name), child);
    }

    pub async fn signal(&mut self, process_name: &str, process_signal: i32) {
        info!(
            "ChildProcessHandle signal {} {:?}",
            process_name, process_signal
        );

        let child_op = self.state.child_processes.get(process_name);
        match child_op {
            Some(child) => {
                unsafe { kill(child.id().unwrap() as i32, process_signal) };
                //signal::kill(Pid::from_raw(), process_signal).unwrap();
            }
            None => {
                info!("process not found")
            }
        }
    }
}

#[async_trait]
impl ServiceHandler for ChildProcessHandle {
    async fn start(&mut self) -> Result<bool> {
        // Start if device is provisioned
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
