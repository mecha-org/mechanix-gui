use std::env;

use crate::{manager::ManagerProxy, user::UserProxy};
use anyhow::Result;
use session::SessionProxy;

pub mod manager;
pub mod session;
pub mod user;

pub async fn get_current_session() -> Result<SessionProxy<'static>> {
    // let user_path = manager.get_user_by_pid(std::process::id()).await?;

    let mut tty = "".to_string();
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--tty" {
            tty = args.get(i + 1).unwrap_or(&String::new()).clone();
        }
    }

    println!("tty args is {:?}", tty);

    if tty.is_empty() {
        let env_vars = env::vars();
        for (key, value) in env_vars {
            if key.to_lowercase() == "tty" && !value.contains("pts") && value.contains("tty") {
                tty = value.clone().replace("/dev/", "");
            }
        }
    }

    println!("tty param is {:?}", tty);

    let connection = zbus::Connection::system().await?;
    if tty.is_empty() && env::var("XDG_SESSION_ID").is_ok() {
        let manager = ManagerProxy::new(&connection).await?;
        let path = manager
            .get_session(env::var("XDG_SESSION_ID").unwrap())
            .await?;
        println!("current session path {:?}", path);
        let session = SessionProxy::builder(&connection)
            .path(path.clone())?
            .build()
            .await?;
        println!("session tty of current process {:?}", session.tty().await);
        return Ok(session);
    }

    let user = UserProxy::builder(&connection)
        // .path(user_path)?
        .build()
        .await?;

    let sessions = user.sessions().await?;
    let mut session_path: String = "".to_string();
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
    println!("session tty {:?}", session.tty().await);
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
