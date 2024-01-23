use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

#[async_trait]
pub trait ServiceHandler {
    fn get_status(&self) -> Result<ServiceStatus>;
    fn is_stopped(&self) -> Result<bool>;
    fn is_started(&self) -> Result<bool>;
    async fn start(&mut self) -> Result<bool>;
    async fn stop(&mut self) -> Result<bool>;
}
