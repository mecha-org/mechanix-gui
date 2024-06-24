use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::types::{Params, PolkitError, PolkitEvent};
use mechanix_system_dbus_client::security::Security;
use policykit::{authority::AuthorityProxy, types::Identity};
use tokio::sync::{mpsc, oneshot};
use zbus::{interface, zvariant};
pub struct PolkitAgentInterface {
    pub sender: mpsc::Sender<PolkitEvent>,
}

#[interface(name = "org.freedesktop.PolicyKit1.AuthenticationAgent")]
impl PolkitAgentInterface {
    async fn begin_authentication(
        &self,
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
        cookie: String,
        identities: Vec<Identity>,
    ) -> Result<(), PolkitError> {
        println!("Agent::begin_authentication() ",);

        println!("Sending PolkitEvent::CreateDialog");
        let (response_tx, response_rx) = oneshot::channel();
        let res = self
            .sender
            .send(PolkitEvent::CreateDialog(Params {
                action_id,
                message,
                icon_name: Some(icon_name),
                details,
                cookie,
                response_tx: Arc::new(Mutex::new(Some(response_tx))),
                identities,
            }))
            .await;
        println!("Response PolkitEvent::CreateDialog {:?}", res);
        if let Err(e) = &res {
            println!("Error while sending event {:?}", e.to_string());
            return Err(PolkitError::Cancelled);
        };
        println!("Sent PolkitEvent::CreateDialog");
        let res = response_rx.await.unwrap();
        println!("Response UI {:?}", res);
        res
    }

    async fn cancel_authentication(&self, cookie: String) -> Result<(), PolkitError> {
        let _ = self.sender.send(PolkitEvent::CancelDialog { cookie }).await;
        Ok(())
    }
}
