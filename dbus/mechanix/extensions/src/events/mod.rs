use serde::{Deserialize, Serialize};

use crate::device::Device;

#[derive(Debug,Serialize,Deserialize)]
pub enum ExtensionServiceEvent {
    Added(Device),
    Removed(Device),
}

