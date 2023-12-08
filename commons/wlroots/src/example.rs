use tokio::sync::{mpsc, oneshot};
use wlr_toplevel_handler::{WlrToplevelHandler, WlrToplevelHandlerOptions, WlrToplevelHandlerMessage};
mod errors;
mod wlr_toplevel_handler;

#[tokio::main]
async fn main() {
    // let mut wl_toplevel_client = toplevel::WlrToplevelManagement::new().await.expect("error from wlroots");

    // let thread_1 = tokio::spawn(async move {
    //     let info = wl_toplevel_client.get_info().await.expect("error from wlroots");
    //     println!("printing top level - {:?}", info.title);
    // });

    let (toplevel_message_tx, toplevel_message_rx) = mpsc::channel(32);
    let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(32);

    let mut wlr_toplevel_handler = WlrToplevelHandler::new(WlrToplevelHandlerOptions { toplevel_event_tx });

    let wlr_thread = tokio::spawn(async move {
        let _ = wlr_toplevel_handler.run(toplevel_message_rx).await;
    });

    while let Some(message) = toplevel_event_rx.recv().await {
        println!("main: toplevel event = {:?}", message);
        
        // get active window/toplevel title
        let (tx, rx) = oneshot::channel();
        let _ = toplevel_message_tx
            .send(WlrToplevelHandlerMessage::GetActiveToplevelTitle { reply_to: tx })
            .await;

        let res = rx.await.expect("no reply from service");

        println!("main: active window title is {}", res.unwrap().unwrap());

        // get active window/toplevel app_id
        let (tx, rx) = oneshot::channel();
        let _ = toplevel_message_tx
            .send(WlrToplevelHandlerMessage::GetActiveToplevelAppId { reply_to: tx })
            .await;

        let res = rx.await.expect("no reply from service");

        println!("main: active window app_id is {}", res.unwrap().unwrap());

        // get all open toplevels
        let (tx, rx) = oneshot::channel();
        let _ = toplevel_message_tx
            .send(WlrToplevelHandlerMessage::GetToplevels { reply_to: tx })
            .await;

        let res = rx.await.expect("no reply from service");

        println!("main: open windows are {:?}", res.unwrap());
    }

    wlr_thread.await.unwrap();
}
