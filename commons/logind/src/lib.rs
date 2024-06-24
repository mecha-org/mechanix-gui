use std::env;

use crate::{manager::ManagerProxy, user::UserProxy};
use anyhow::Result;
use session::SessionProxy;

pub mod manager;
pub mod session;
pub mod user;

pub async fn get_current_session() -> Result<SessionProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let manager = ManagerProxy::new(&connection).await?;
    // let user_path = manager.get_user_by_pid(std::process::id()).await?;
    let user = UserProxy::builder(&connection)
        // .path(user_path)?
        .build()
        .await?;

    let mut session_path: String = "".to_string();
    let mut tty = "".to_string();
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--tty" {
            tty = args.get(i + 1).unwrap_or(&String::new()).clone();
        }
    }
    if tty.is_empty() {
        let env_vars = env::vars();
        for (key, value) in env_vars {
            if key.to_lowercase() == "tty" && !value.contains("pts") && value.contains("tty") {
                tty = value.clone().replace("/dev/", "");
            }
        }
    }

    let sessions = user.sessions().await?;
    for (_, path) in sessions {
        let session = SessionProxy::builder(&connection)
            .path(&path)?
            .build()
            .await?;
        let session_tty = session.tty().await?;

        if tty == session_tty {
            session_path = path.to_string();
            break;
        }
    }
    let session = SessionProxy::builder(&connection)
        .path(session_path)?
        .build()
        .await?;
    Ok(session)
}

pub async fn session_unlock() -> Result<bool> {
    let session = get_current_session().await?;
    let _ = session.unlock().await;
    Ok(true)
}

pub async fn session_lock() -> Result<bool> {
    let session = get_current_session().await?;
    println!("session_lock {:?}", session.id().await.unwrap(),);
    session.lock().await?;
    Ok(true)
}
