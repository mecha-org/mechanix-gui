use std::collections::HashMap;

use crate::dbus::interfaces::agent::PolkitAgentInterface;
use crate::types::PolkitEvent;
use logind::get_current_session;
use policykit::{authority::AuthorityProxy, types::Subject};
use tokio::sync::mpsc;
use zbus::Connection;
const CUSTOM_AGENT_OBJECT_PATH: &str = "/org/mechanix/agent/Polkit";
pub async fn register_polkit_agent(sender: mpsc::Sender<PolkitEvent>) {
    let connection = Connection::system().await;

    if let Err(e) = connection {
        println!("Error in connecting to system bus: {}", e);
        return;
    }

    let connection = connection.unwrap();

    let agent = PolkitAgentInterface { sender };

    let served = connection
        .object_server()
        .at(CUSTOM_AGENT_OBJECT_PATH, agent)
        .await;

    if let Err(e) = served {
        println!("Error in serving polkit agent system bus: {}", e);
        return;
    }

    let served = served.unwrap();

    println!("polkit agent served {:?}", served);

    if !served {
        return;
    }

    let session = get_current_session().await;
    if let Err(e) = session {
        println!("Error in getting current session: {}", e);
        return;
    };

    let session = session.unwrap();

    let session_id = session.id().await;
    if let Err(e) = session_id {
        println!("Error in getting session id: {}", e);
        return;
    }
    let session_id = session_id.unwrap();

    let mut subject_details = HashMap::new();
    subject_details.insert("session-id", session_id.into());
    let subject = Subject {
        subject_kind: "unix-session",
        subject_details,
    };

    let authority = AuthorityProxy::new(&connection).await;
    if let Err(e) = authority {
        println!("Error in getting authority: {}", e);
        return;
    }
    let authority = authority.unwrap();

    println!("registering agent");

    let regiter = authority
        .register_authentication_agent(subject, "en_US", CUSTOM_AGENT_OBJECT_PATH)
        .await;
    if let Err(e) = regiter {
        println!("Error in registering agent: {}", e);
        return;
    }
    println!("Agent registered");
    loop {
        std::future::pending::<()>().await;
    }
}
