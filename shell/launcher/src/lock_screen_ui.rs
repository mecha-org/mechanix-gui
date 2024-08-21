use std::sync::Arc;
use std::sync::RwLock;

use crate::AppParams;
use crate::{lock_gui, UiParams};
use mctk_core::{
    component::{self, Component, Message, RenderContext, RootComponent},
    lay,
    layout::Direction,
    msg, node, rect,
    reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Sender},
    renderables::Renderable,
    size, size_pct,
    style::Styled,
    txt,
    widgets::{Button, Div},
    Color,
};
use mctk_smithay::session_lock::lock_window::SessionLockWindowParams;
use mctk_smithay::session_lock::lock_window::{SessionLockMessage, SessionLockWindow};
use mctk_smithay::WindowMessage;
use mctk_smithay::WindowOptions;

use mctk_macros::{component, state_component_impl};

pub fn launch_lockscreen(
    lock_window_tx: Arc<RwLock<Option<Sender<WindowMessage>>>>,
    ui_params: UiParams,
) -> anyhow::Result<()> {
    let UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
    } = ui_params;

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let (session_lock_tx, session_lock_rx) = calloop::channel::channel();
    // let (status_bar_channel, status_bar_receiver) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) =
        SessionLockWindow::open_blocking::<lock_gui::Lockscreen, AppParams>(
            SessionLockWindowParams {
                session_lock_tx,
                session_lock_rx,
                window_opts,
                fonts,
                assets,
                svgs,
            },
            AppParams { app_channel: None },
        );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    {
        let mut lock_window_tx = lock_window_tx.write().unwrap();
        *lock_window_tx = Some(window_tx_2);
    }

    // let _ = handle.insert_source(status_bar_receiver, move |event, _, _| {
    //     let _ = match event {
    //         // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
    //         calloop::channel::Event::Msg(msg) => match msg {
    //             StatusBarMessage::Clock { current_time } => {
    //                 //println!("StatusBarMessage::Clock {:?}", current_time);
    //                 let _ = window_tx_2.clone().send(WindowMessage::Send {
    //                     message: msg!(Message::Clock { current_time }),
    //                 });
    //             }
    //             StatusBarMessage::Wireless { status } => {
    //                 let _ = window_tx_2.clone().send(WindowMessage::Send {
    //                     message: msg!(Message::Wireless { status }),
    //                 });
    //             }
    //             StatusBarMessage::Bluetooth { status } => {
    //                 let _ = window_tx_2.clone().send(WindowMessage::Send {
    //                     message: msg!(Message::Bluetooth { status }),
    //                 });
    //             }
    //             StatusBarMessage::Battery { level, status } => {
    //                 let _ = window_tx_2.clone().send(WindowMessage::Send {
    //                     message: msg!(Message::Battery { level, status }),
    //                 });
    //             }

    //             _ => (),
    //         },
    //         calloop::channel::Event::Closed => {}
    //     };
    // });

    // init_services(settings.clone(), status_bar_channel);

    loop {
        event_loop.dispatch(None, &mut app).unwrap();

        if app.is_exited {
            break;
        }
    }
    //End
    println!("lock screen exited");
    {
        let mut lock_window_tx = lock_window_tx.write().unwrap();
        *lock_window_tx = None;
    }
    Ok(())
}
