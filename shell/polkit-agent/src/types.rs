use policykit::types::Identity;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use zbus::zvariant;

use tokio::sync::oneshot;

#[derive(Debug)]
pub enum PolkitEvent {
    CreateDialog(Params),
    CancelDialog { cookie: String },
}

#[allow(dead_code)]
#[derive(Clone, Debug, zbus::DBusError)]
#[zbus(prefix = "org.freedesktop.PolicyKit1.Error")]
pub enum PolkitError {
    Failed,
    Cancelled,
    NotSupported,
    NotAuthorized,
    CancellationIdNotUnique,
}

#[derive(Debug)]
pub struct Params {
    pub action_id: String,
    pub message: String,
    pub icon_name: Option<String>,
    pub details: HashMap<String, String>,
    pub cookie: String,
    pub response_tx: Arc<Mutex<Option<oneshot::Sender<Result<(), PolkitError>>>>>,
    pub identities: Vec<Identity>,
}
