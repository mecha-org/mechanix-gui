use anyhow::Result;
use command::spawn_command;
use futures::StreamExt;
use logind::{
    get_current_session,
    session::{self, SessionProxy},
    session_unlock,
};
use tokio::{select, spawn};

use crate::{
    handlers::{idle_notify::IdleNotifyHandlerClient, lock_button::LockButtonHandler},
    settings::DesktopServerSettings,
};

pub struct SessionHandler {
    // is_session_locked: bool,
    settings: DesktopServerSettings,
}
impl SessionHandler {
    pub fn new(settings: DesktopServerSettings) -> Self {
        Self {
            // is_session_locked: false,
            settings,
        }
    }

    pub async fn run(mut self) {
        println!("SessionHandler::run()");

        //instantiate idle notify
        let idle_notify_handler = IdleNotifyHandlerClient::new(self.settings.idle_notify.clone());

        //instantiate button handler
        let lock_button_handler = LockButtonHandler::new(self.settings.lock_button);

        let idle_notify_t = tokio::spawn(idle_notify_handler.run());

        let lock_button_t = tokio::spawn(lock_button_handler.run());

        // let session = get_current_session().await.unwrap();
        // let mut lock = session.receive_lock().await.unwrap();
        // let mut unlock = session.receive_unlock().await.unwrap();

        // let lock_command = self.settings.session.run_commands.lock_screen.clone();
        // let session_event_t = tokio::spawn(async move {
        //     //Listen for events
        //     loop {
        //         select! {
        //             _ =  lock.next() =>  {
        //                 println!("logind lock {:?}", self.is_session_locked);
        //                 if self.is_session_locked {
        //                     continue;
        //                 }

        //                 if let Ok(session_locked) = lock_session(lock_command.clone()).await {
        //                     self.is_session_locked = session_locked;
        //                 };
        //             }, _ = unlock.next() => {
        //                 println!("logind unlock");
        //                 self.is_session_locked = false;
        //             }
        //         }
        //     }
        // });

        idle_notify_t.await.unwrap();
        lock_button_t.await.unwrap();
        // session_event_t.await.unwrap();
    }
}

// async fn lock_session(lock_command: String) -> Result<bool> {
//     let _ = spawn(async move {
//         if !lock_command.is_empty() {
//             let mut args: Vec<String> = vec!["-c".to_string()];
//             args.push(lock_command.clone());
//             let res = spawn_command("sh".to_string(), args);
//             println!("spawn_command res {:?}", res);
//         }
//     })
//     .await;
//     Ok(true)
// }
