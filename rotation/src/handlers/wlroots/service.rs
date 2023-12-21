use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{error, info};

use super::handler::{RotationDirection, WlrootsHandlerMessage};

pub struct DispatchRotationParams {
    pub wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>,
}

pub fn rotate(direction: RotationDirection) -> Result<bool> {
    info!("rotate direction:  {:?}", direction);

    Ok(true)
}
