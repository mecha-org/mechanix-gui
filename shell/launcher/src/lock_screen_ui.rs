use crate::gui::Message;
use crate::init_services;
use crate::AppMessage;
use crate::AppParams;
use crate::BatteryMessage;
use crate::BluetoothMessage;
use crate::InitServicesParams;
use crate::WirelessMessage;
use crate::{lock_gui, UiParams};
use logind::get_current_session;
use mctk_core::{
    msg,
    reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Event},
};
use mctk_smithay::session_lock::lock_window::SessionLockMessage;
use mctk_smithay::session_lock::lock_window::SessionLockWindow;
use mctk_smithay::session_lock::lock_window::SessionLockWindowParams;
use mctk_smithay::WindowMessage;
use mctk_smithay::WindowOptions;

pub fn launch_lockscreen(ui_params: UiParams) -> anyhow::Result<()> {
    let UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
        ..
    } = ui_params;

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let (session_lock_tx, session_lock_rx) = calloop::channel::channel();
    //subscribe to events channel
    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) =
        SessionLockWindow::open_blocking::<lock_gui::Lockscreen, AppParams>(
            SessionLockWindowParams {
                session_lock_tx: session_lock_tx.clone(),
                session_lock_rx,
                window_opts,
                fonts,
                assets,
                svgs,
            },
            AppParams {
                app_channel: Some(app_channel_tx.clone()),
                ..Default::default()
            },
        );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();

    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Clock { date, time } => {
                    //println!("AppMessage::Clock {:?}", current_time);
                    println!("sending clock message to homescreen");
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Clock {
                            date: date.clone(),
                            time: time.clone()
                        }),
                    });
                }
                AppMessage::Wireless { message } => match message {
                    WirelessMessage::Status { status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Wireless { status }),
                        });
                    }
                    _ => (),
                },
                AppMessage::Bluetooth { message } => match message {
                    BluetoothMessage::Status { status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Bluetooth { status }),
                        });
                    }
                    _ => (),
                },
                AppMessage::Battery { message } => match message {
                    BatteryMessage::Status { level, status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Battery { level, status }),
                        });
                    }
                },
                AppMessage::Unlock => {
                    let _ = session_lock_tx.send(SessionLockMessage::Unlock);
                    zbus::block_on(async move {
                        let session = get_current_session().await.unwrap();
                        let _ = session.set_locked_hint(false).await;
                    });
                }
                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });

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

    init_services(InitServicesParams {
        settings,
        app_channel: app_channel_tx,
        ..Default::default()
    });

    loop {
        event_loop.dispatch(None, &mut app).unwrap();

        if app.is_exited {
            break;
        }
    }
    //End
    Ok(())
}
