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
#[derive(Debug, Clone)]
enum HelloEvent {
    ButtonPressed {
        name: String,
    },
    TextBox {
        name: String,
        value: String,
        update_type: String,
    },
    Exit,
}

#[derive(Debug, Default)]
pub struct LockScreenState {
    session_lock_sender: Option<Sender<SessionLockMessage>>,
}

#[component(State = "LockScreenState")]
#[derive(Debug, Default)]
pub struct LockScreen {}

#[state_component_impl(LockScreenState)]
impl Component for LockScreen {
    fn init(&mut self) {
        self.state = Some(LockScreenState {
            session_lock_sender: None,
        });
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        None
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        println!(
            "LockScreen was sent: {:?}",
            message.downcast_ref::<HelloEvent>()
        );
        match message.downcast_ref::<HelloEvent>() {
            Some(HelloEvent::ButtonPressed { name }) => {
                println!("{}", name);
            }
            Some(HelloEvent::Exit) => {
                println!("button clicked");
                if let Some(session_lock_tx) = self.state_ref().session_lock_sender.clone() {
                    let _ = session_lock_tx.send(SessionLockMessage::Unlock);
                };
            }
            _ => (),
        }
        vec![]
    }

    fn view(&self) -> Option<mctk_core::Node> {
        Some(
            node!(
                Div::new().bg(Color::LIGHT_GREY),
                lay![
                    size: size_pct!(100.0),
                    direction: Direction::Column,
                    axis_alignment: mctk_core::layout::Alignment::Center,
                    cross_alignment: mctk_core::layout::Alignment::Center
                ]
            )
            .push(node!(
                Button::new(txt!("Unlock"))
                    .on_click(Box::new(|| msg!(HelloEvent::Exit)))
                    .on_double_click(Box::new(|| msg!(HelloEvent::ButtonPressed {
                        name: "Double clicked".to_string()
                    })))
                    .style("color", Color::rgb(255., 0., 0.))
                    .style("background_color", Color::rgb(255., 255., 255.))
                    .style("active_color", Color::rgb(200., 200., 200.))
                    .style("font_size", 24.0),
                lay![size: size!(180.0, 180.0), margin: [0., 0., 20., 0.]]
            )),
        )
    }
}

impl RootComponent<AppParams> for LockScreen {
    fn root(&mut self, window: &dyn std::any::Any, app_params: &dyn std::any::Any) {
        let session_lock_window = window.downcast_ref::<SessionLockWindow>();
        if session_lock_window.is_some() {
            self.state_mut().session_lock_sender = Some(session_lock_window.unwrap().sender());
        }
    }
}
