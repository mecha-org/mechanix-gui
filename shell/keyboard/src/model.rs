use std::collections::{HashMap, HashSet};

use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::{
    context::Context,
    prelude::smithay_client_toolkit::{reexports::calloop, shell::wlr_layer},
};
use mctk_macros::Model;
use mctk_smithay::{
    layer_shell::{layer_surface::LayerOptions, layer_window::LayerWindowMessage},
    WindowMessage,
};
use tokio::{runtime::Runtime, sync::mpsc};
use wayland_protocols_async::{
    zwp_input_method_v2::handler::{
        ContentPurpose, InputMethodEvent, InputMethodHandler, InputMethodMessage,
    },
    zwp_virtual_keyboard_v1::handler::{KeyMotion, VirtualKeyboardHandler, VirtualKeyboardMessage},
};

use crate::{
    action::{Modifier, Modifiers},
    layout::KeyCode,
    settings::TrieConfigs,
    trie::util::get_trie,
};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref Keyboard: KeyboardModel = KeyboardModel {
        is_running: Context::new(false),
        visible: Context::new(false),
        input_method_msg_tx: Context::new(None),
        virtual_keyboard_msg_tx: Context::new(None),
        trie_configs: Context::new(None),
        purpose: Context::new(None),
        layouts_map: Context::new(HashMap::new()),
        window_tx: Context::new(None),
        maximize: Context::new(true),
        layer_tx: Context::new(None),
        current_view: Context::new(String::from("base")),
        suggestions: Context::new(Vec::new()),
        suggested_for: Context::new(String::new()),
        next_char_prob: Context::new(HashMap::new())
    };
}

#[derive(Model)]
pub struct KeyboardModel {
    is_running: Context<bool>,
    input_method_msg_tx: Context<Option<mpsc::Sender<InputMethodMessage>>>,
    virtual_keyboard_msg_tx: Context<Option<mpsc::Sender<VirtualKeyboardMessage>>>,
    pub maximize: Context<bool>,
    pub trie_configs: Context<Option<TrieConfigs>>,
    pub visible: Context<bool>,
    pub purpose: Context<Option<ContentPurpose>>,
    pub layouts_map: Context<HashMap<ContentPurpose, (i32, u32)>>,
    pub window_tx: Context<Option<calloop::channel::Sender<WindowMessage>>>,
    pub layer_tx: Context<Option<calloop::channel::Sender<LayerWindowMessage>>>,
    pub current_view: Context<String>,
    pub suggestions: Context<Vec<String>>,
    pub suggested_for: Context<String>,
    pub next_char_prob: Context<HashMap<String, f64>>,
}

impl KeyboardModel {
    pub fn get() -> &'static Self {
        &Keyboard
    }

    pub fn suggestion_pressed(suggestion: String) {
        if let Some(input_method_msg_tx) = Self::get().input_method_msg_tx.get().clone() {
            let suggested_for = Self::get().suggested_for.get().clone();
            RUNTIME.spawn(async move {
                let _ = input_method_msg_tx
                    .send(InputMethodMessage::DeleteSurroundingText {
                        before_length: suggested_for.len() as u32,
                        after_length: 0,
                    })
                    .await;
                let _ = input_method_msg_tx
                    .send(InputMethodMessage::CommitString { text: suggestion })
                    .await;
                let _ = input_method_msg_tx.send(InputMethodMessage::Commit).await;
            });
        }
    }

    pub fn key_pressed(keycode: KeyCode) {
        if let Some(virtual_keyboard_msg_tx) = Self::get().virtual_keyboard_msg_tx.get().clone() {
            RUNTIME.spawn(async move {
                let _ = virtual_keyboard_msg_tx
                    .send(VirtualKeyboardMessage::Key {
                        keycode: keycode.code - 8,
                        keymotion: KeyMotion::Press,
                    })
                    .await;
                let _ = virtual_keyboard_msg_tx
                    .send(VirtualKeyboardMessage::Key {
                        keycode: keycode.code - 8,
                        keymotion: KeyMotion::Release,
                    })
                    .await;
            });
        };
    }

    pub fn erase() {
        if let Some(virtual_keyboard_msg_tx) = Self::get().virtual_keyboard_msg_tx.get().clone() {
            RUNTIME.spawn(async move {
                let _ = virtual_keyboard_msg_tx
                    .send(VirtualKeyboardMessage::Key {
                        keycode: 21 - 8,
                        keymotion: KeyMotion::Press,
                    })
                    .await;
                let _ = virtual_keyboard_msg_tx
                    .send(VirtualKeyboardMessage::Key {
                        keycode: 21 - 8,
                        keymotion: KeyMotion::Release,
                    })
                    .await;
            });
        };
    }

    pub fn apply_modifiers(mods: HashSet<Modifier>) {
        if let Some(virtual_keyboard_msg_tx) = Self::get().virtual_keyboard_msg_tx.get().clone() {
            RUNTIME.spawn(async move {
                let raw_modifiers = mods
                    .iter()
                    .map(|m| match m {
                        Modifier::Control => Modifiers::CONTROL,
                        Modifier::Alt => Modifiers::MOD1,
                        Modifier::Mod4 => Modifiers::MOD4,
                    })
                    .fold(Modifiers::empty(), |m, n| m | n);

                RUNTIME.spawn(async move {
                    let _ = virtual_keyboard_msg_tx
                        .send(VirtualKeyboardMessage::SetModifiers {
                            depressed: raw_modifiers.bits() as u32,
                            latched: 0,
                            locked: 0,
                        })
                        .await;
                });
            });
        };
    }

    pub fn init() {
        RUNTIME.spawn(async {
            Self::run_input_method_handler();
            Self::run_keyboard_hanlder();
        });
    }

    pub fn maximize() {
        Self::get().maximize.set(true);
        Self::get().current_view.set(String::from("base"));
        Self::recalculate();
    }

    pub fn minimize() {
        Self::get().current_view.set(String::from("minimize"));
        Self::get().maximize.set(false);
        Self::recalculate();
    }

    fn run_keyboard_hanlder() {
        let layouts = Self::get().layouts_map.get().clone();
        let default_layout = layouts.get(&ContentPurpose::Normal).unwrap().clone();
        RUNTIME.spawn(async move {
            // create mpsc channel for receiving events from the virtual_keyboard handler
            let (virtual_keyboard_event_tx, virtual_keyboard_event_rx) = mpsc::channel(128);
            let (virtual_keyboard_msg_tx, virtual_keyboard_msg_rx) = mpsc::channel(128);
            Self::get()
                .virtual_keyboard_msg_tx
                .set(Some(virtual_keyboard_msg_tx));
            let mut virtual_keyboard_handler: VirtualKeyboardHandler = VirtualKeyboardHandler::new(
                default_layout.0,
                default_layout.1,
                virtual_keyboard_event_tx,
            );

            // start the virtual_keyboard handler
            let _ = virtual_keyboard_handler.run(virtual_keyboard_msg_rx).await;
        });
    }

    fn run_input_method_handler() {
        RUNTIME.spawn(async move {
            // create mpsc channel for receiving events from the input_method handler
            let (input_method_tx, mut input_method_event_rx) = mpsc::channel(128);

            // create the handler instance
            let mut input_method_handler = InputMethodHandler::new(input_method_tx);

            // start the input_method handler
            RUNTIME.spawn(async move {
                let (input_method_msg_tx, input_method_msg_rx) = mpsc::channel(128);
                Self::get()
                    .input_method_msg_tx
                    .set(Some(input_method_msg_tx));
                input_method_handler.run(input_method_msg_rx).await;
            });
            let trie_configs = Self::get().trie_configs.get().clone().unwrap();
            // receive all input_method events
            if let (Some(raw_file), Some(cached_file)) =
                (trie_configs.raw_file, trie_configs.cached_file)
            {
                let input_method_event_t = RUNTIME.spawn(async move {
                    let trie = get_trie(&raw_file, &cached_file);

                    loop {
                        if let Some(msg) = input_method_event_rx.recv().await {
                            // println!("InputHandler::run_input_handler() {:?}", msg);
                            match msg {
                                InputMethodEvent::Activate => {
                                    println!("InputMethodEvent::Activate");
                                    Self::get().visible.set(true);
                                    Self::recalculate();
                                    //Send message to window to show UI
                                    // let _ = app_channel.send(AppMessage::Show);
                                }
                                InputMethodEvent::Deactivate => {
                                    println!("InputMethodEvent::Deactivate");
                                    Self::get().visible.set(false);
                                    Self::get().purpose.set(None);
                                    Self::recalculate();
                                    Self::get().maximize.set(true);
                                    Self::get().current_view.set(String::from("base"));

                                    //Send message to window to hide UI
                                    // let _ = app_channel.send(AppMessage::Hide);
                                }
                                InputMethodEvent::SurroundingText {
                                    text,
                                    cursor,
                                    anchor,
                                } => {
                                    //Send message to window to update UI with text, cursor, and anchor positions
                                    let words = &text.as_str()[0..cursor as usize].split(" ");
                                    if let Some(last) = words.clone().last() {
                                        let suggestions = trie.search(last);
                                        let next_char_prob = trie.next_char_probabilities(last);
                                        Self::get().suggestions.set(suggestions);
                                        Self::get().suggested_for.set(last.to_ascii_lowercase());
                                        Self::get().next_char_prob.set(next_char_prob);
                                        // let _ = app_channel.send(AppMessage::SuggestionsChanged {
                                        //     suggestions,
                                        //     suggested_for: last.to_string(),
                                        //     next_char_prob,
                                        // });
                                    }
                                }
                                InputMethodEvent::ContentType { hint, purpose } => {
                                    //Use purpose to change layout

                                    if let Ok(purpose) = purpose {
                                        Self::get().purpose.set(Some(purpose));
                                        Self::recalculate();
                                        // let _ =
                                        //     app_channel.send(AppMessage::ContentInfo { purpose });
                                    }
                                }

                                _ => (),
                            }
                        };
                    }
                });
                let _ = input_method_event_t.await.unwrap();
            };
        });
    }

    fn recalculate() {
        let mut layer_shell_opts = LayerOptions {
            anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
            layer: wlr_layer::Layer::Top,
            keyboard_interactivity: wlr_layer::KeyboardInteractivity::None,
            namespace: Some(String::from("mechanix.shell.keyboard")),
            zone: 0 as i32,
        };
        let purpose = Self::get().purpose.get().clone();
        if purpose.is_none() {
            if let (Some(window_tx), Some(layer_tx)) = (
                Self::get().window_tx.get().clone(),
                Self::get().layer_tx.get().clone(),
            ) {
                layer_shell_opts.anchor =
                    wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM;
                layer_shell_opts.zone = 0 as i32;
                layer_shell_opts.layer = wlr_layer::Layer::Bottom;
                let _ = layer_tx
                    .clone()
                    .send(LayerWindowMessage::ReconfigureLayerOpts {
                        opts: layer_shell_opts.clone(),
                    });
                let _ = window_tx.send(WindowMessage::Resize {
                    width: 1 as u32,
                    height: 1 as u32,
                });
            }
            return;
        }

        //Check if visible
        let visible = Self::get().visible.get().clone();
        if !visible {
            if let (Some(window_tx), Some(layer_tx)) = (
                Self::get().window_tx.get().clone(),
                Self::get().layer_tx.get().clone(),
            ) {
                layer_shell_opts.anchor =
                    wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM;
                layer_shell_opts.zone = 0 as i32;
                layer_shell_opts.layer = wlr_layer::Layer::Bottom;
                let _ = layer_tx
                    .clone()
                    .send(LayerWindowMessage::ReconfigureLayerOpts {
                        opts: layer_shell_opts.clone(),
                    });
                let _ = window_tx.send(WindowMessage::Resize {
                    width: 1 as u32,
                    height: 1 as u32,
                });
            }
            return;
        }

        if !Self::get().maximize.get().clone() {
            if let (Some(window_tx), Some(layer_tx)) = (
                Self::get().window_tx.get().clone(),
                Self::get().layer_tx.get().clone(),
            ) {
                let _ = window_tx.send(WindowMessage::Resize {
                    width: 80 as u32,
                    height: 60 as u32,
                });

                layer_shell_opts.anchor = wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM;
                layer_shell_opts.zone = 0 as i32;
                layer_shell_opts.layer = wlr_layer::Layer::Overlay;
                let _ = layer_tx
                    .clone()
                    .send(LayerWindowMessage::ReconfigureLayerOpts {
                        opts: layer_shell_opts.clone(),
                    });
            }
            return;
        }

        let purpose = purpose.unwrap();
        if let Some(virtual_keyboard_msg_tx) = Self::get().virtual_keyboard_msg_tx.get().clone() {
            RUNTIME.spawn(async move {
                let layouts_map = Self::get().layouts_map.get().clone();
                if let Some((keymap_raw_fd, keymap_size)) = layouts_map.get(&purpose) {
                    let _ = virtual_keyboard_msg_tx
                        .send(VirtualKeyboardMessage::SetKeymap {
                            keymap_raw_fd: *keymap_raw_fd,
                            keymap_size: *keymap_size,
                        })
                        .await;
                };
            });
        }

        let (mut width, mut height) = (1., 1.);

        println!(
            "purpose on_change() {:?} {:?}",
            purpose,
            KeyboardModel::get().visible.get().clone()
        );
        match purpose {
            ContentPurpose::Normal | ContentPurpose::Alpha | ContentPurpose::Name => {
                width = 480.;
                height = 244.;
            }

            ContentPurpose::Terminal
            | ContentPurpose::Email
            | ContentPurpose::Url
            | ContentPurpose::Password
            | ContentPurpose::Digits
            | ContentPurpose::Number
            | ContentPurpose::Phone
            | ContentPurpose::Pin
            | ContentPurpose::Date
            | ContentPurpose::Time
            | ContentPurpose::Datetime => {
                width = 480.;
                height = 210.;
            }
            _ => {
                width = 480.;
                height = 210.;
            }
        }

        if let (Some(window_tx), Some(layer_tx)) = (
            Self::get().window_tx.get().clone(),
            Self::get().layer_tx.get().clone(),
        ) {
            let _ = window_tx.send(WindowMessage::Resize {
                width: width as u32,
                height: height as u32,
            });
            layer_shell_opts.anchor =
                wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM;
            layer_shell_opts.zone = height as i32;
            layer_shell_opts.layer = wlr_layer::Layer::Overlay;
            let _ = layer_tx
                .clone()
                .send(LayerWindowMessage::ReconfigureLayerOpts {
                    opts: layer_shell_opts.clone(),
                });
        }

        // let _ = window_tx_4.clone().send(WindowMessage::Resize {
        //     width: width as u32,
        //     height: height as u32,
        // });
        // let _ = window_tx_4.clone().send(WindowMessage::Send {
        //     message: msg!(Message::ContentInfo { purpose }),
        // });
    }

    // pub fn run() {
    //     if *KeyboardModel::get().is_running.get() {
    //         return;
    //     }

    //     KeyboardModel::get().is_running.set(true);
    //     // Self::run_input_method_handler();
    // }
}
