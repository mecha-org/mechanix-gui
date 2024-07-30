mod action;
mod components;
mod constants;
mod errors;
mod gui;
mod layout;
mod settings;
mod trie;

use gui::Keyboard;
use mctk_core::{
    msg,
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{
                self,
                channel::{Event, Sender},
            },
            shell::wlr_layer,
        },
    },
    types::AssetParams,
};
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};
use std::io::{Seek, Write};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{collections::HashMap, io::SeekFrom};
use std::{io::Read, os::fd::IntoRawFd};
use tempfile::tempfile;
use tokio::{
    runtime::Builder,
    sync::mpsc::{self, Receiver},
};
use trie::util::get_trie;
use wayland_protocols_async::{
    zwp_input_method_v2::handler::{InputMethodEvent, InputMethodHandler, InputMethodMessage},
    zwp_virtual_keyboard_v1::handler::{KeyMotion, VirtualKeyboardHandler, VirtualKeyboardMessage},
};

use crate::gui::Message;
use settings::{Icons, KeyboardSettings, TrieConfigs};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[derive(Debug)]
enum AppMessage {
    Show,
    Hide,
    TextkeyPressed {
        keycode: crate::layout::KeyCode,
    },
    SuggestionsChanged {
        suggestions: Vec<String>,
        suggested_for: String,
        next_char_prob: HashMap<String, f64>,
    },
    SuggestionPressed {
        suggestion: String,
        suggested_for: String,
    },
    Erase,
}

fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);
            KeyboardSettings::default()
        }
    };

    let window_opts = WindowOptions {
        height: 1 as u32,
        width: 1 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();
    let Icons {
        backspace,
        enter,
        shift,
        symbolic,
    } = settings.icons.clone();

    svgs.insert("edit-clear-symbolic".to_string(), backspace);

    svgs.insert("key-enter".to_string(), enter);

    svgs.insert("key-shift".to_string(), shift);

    svgs.insert("keyboard-mode-symbolic".to_string(), symbolic);

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.keyboard"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::None,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    //subscribe to events channel
    let (app_channel, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<Keyboard, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel.clone()),
        },
    );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
    let (virtual_keyboard_msg_tx, virtual_keyboard_msg_rx) = mpsc::channel(128);
    // create mpsc channel for interacting with the input_method handler
    let (input_method_msg_tx, input_method_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Show => {
                    let _ = window_tx_2.clone().send(WindowMessage::Resize {
                        width: settings.window.size.0 as u32,
                        height: settings.window.size.1 as u32,
                    });
                }
                AppMessage::Hide => {
                    let _ = window_tx_2.clone().send(WindowMessage::Resize {
                        width: 1,
                        height: 1,
                    });
                }
                AppMessage::TextkeyPressed { keycode } => {
                    println!("AppMessage::TextkeyPressed {:?}", keycode);

                    let virtual_keyboard_msg_tx = virtual_keyboard_msg_tx.clone();
                    futures::executor::block_on(async move {
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
                }
                AppMessage::SuggestionPressed {
                    suggestion,
                    suggested_for,
                } => {
                    let input_method_msg_tx = input_method_msg_tx.clone();
                    futures::executor::block_on(async move {
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
                AppMessage::SuggestionsChanged {
                    suggestions,
                    suggested_for,
                    next_char_prob,
                } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::UpdateSuggestions {
                            suggestions,
                            suggested_for,
                            next_char_prob
                        }),
                    });
                }
                AppMessage::Erase => {
                    let input_method_msg_tx = input_method_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let _ = input_method_msg_tx
                            .send(InputMethodMessage::DeleteSurroundingText {
                                before_length: 1 as u32,
                                after_length: 0,
                            })
                            .await;
                        let _ = input_method_msg_tx.send(InputMethodMessage::Commit).await;
                    });
                }
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(
        app_channel,
        virtual_keyboard_msg_rx,
        input_method_msg_rx,
        settings.clone(),
    );

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}

fn init_services(
    app_channel: Sender<AppMessage>,
    virtual_keyboard_msg_rx: mpsc::Receiver<VirtualKeyboardMessage>,
    input_method_msg_rx: mpsc::Receiver<InputMethodMessage>,
    settings: KeyboardSettings,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let layout_configs = settings.layouts.clone();
        let keyboard_f =
            run_keyboard_hanlder(layout_configs.default.clone(), virtual_keyboard_msg_rx);
        let trie_configs = settings.trie.clone();
        let input_f = InputHandler::new().run_input_handler(
            app_channel.clone(),
            input_method_msg_rx,
            trie_configs,
        );

        runtime
            .block_on(runtime.spawn(async move { tokio::join!(keyboard_f, input_f,) }))
            .unwrap();
    })
}

struct InputHandler {}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {}
    }

    async fn run_input_handler(
        mut self,
        app_channel: Sender<AppMessage>,
        input_method_msg_rx: mpsc::Receiver<InputMethodMessage>,
        trie_configs: TrieConfigs,
    ) {
        // create mpsc channel for receiving events from the input_method handler
        let (input_method_event_tx, mut input_method_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut input_method_handler = InputMethodHandler::new(input_method_event_tx);

        // start the input_method handler
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let _ = runtime.block_on(input_method_handler.run(input_method_msg_rx));
        });

        // receive all input_method events
        if let (Some(raw_file), Some(cached_file)) =
            (trie_configs.raw_file, trie_configs.cached_file)
        {
            let input_method_event_t = tokio::spawn(async move {
                let trie = get_trie(&raw_file, &cached_file);

                loop {
                    if let Some(msg) = input_method_event_rx.recv().await {
                        match msg {
                            InputMethodEvent::Activate => {
                                //Send message to window to show UI
                                let _ = app_channel.send(AppMessage::Show);
                            }
                            InputMethodEvent::Deactivate => {
                                //Send message to window to hide UI
                                let _ = app_channel.send(AppMessage::Hide);
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
                                    let _ = app_channel.send(AppMessage::SuggestionsChanged {
                                        suggestions,
                                        suggested_for: last.to_string(),
                                        next_char_prob,
                                    });
                                }
                            }
                            InputMethodEvent::ContentType { hint, purpose } => {
                                //Use purpose to change layout
                            }

                            _ => (),
                        }
                    };
                }
            });
            let _ = input_method_event_t.await.unwrap();
        };
    }
}

// create mpsc channel for interacting with the virtual_keyboard handler
//let (virtual_keyboard_msg_tx, mut virtual_keyboard_msg_rx) = mpsc::channel(128);

async fn run_keyboard_hanlder(
    layout_path: String,
    virtual_keyboard_msg_rx: mpsc::Receiver<VirtualKeyboardMessage>,
) {
    // create the handler instance
    let (keymap_raw_fd, keymap_size) = load_keymap(layout_path);

    // create mpsc channel for receiving events from the virtual_keyboard handler
    let (virtual_keyboard_event_tx, mut virtual_keyboard_event_rx) = mpsc::channel(128);
    let mut virtual_keyboard_handler: VirtualKeyboardHandler =
        VirtualKeyboardHandler::new(keymap_raw_fd, keymap_size, virtual_keyboard_event_tx);

    // start the virtual_keyboard handler
    let virtual_keyboard_t = tokio::spawn(async move {
        println!("running keyboard handler");
        let _ = virtual_keyboard_handler.run(virtual_keyboard_msg_rx).await;
    });

    println!("init ready");

    let _ = virtual_keyboard_t.await.unwrap();
}
fn load_keymap(layout_path: String) -> (i32, u32) {
    let layout = match crate::layout::Layout::from_file(layout_path) {
        Ok(layout) => layout,
        Err(e) => {
            println!("Error parsing layout {:?}", e);
            panic!("");
        }
    };

    let parsed_layout = layout.clone().build().unwrap();
    let src = parsed_layout.keymaps.get(0).unwrap().as_str();

    // let keymap_size = KEYMAP.len();
    let keymap_size = src.len();
    let keymap_size_u32: u32 = keymap_size.try_into().unwrap(); // Convert it from usize to u32, panics if it is not possible
    let keymap_size_u64: u64 = keymap_size.try_into().unwrap(); // Convert it from usize to u64, panics if it is not possible
                                                                // Create a temporary file
    let mut keymap_file = tempfile().expect("Unable to create tempfile");
    // Allocate the required space in the file first
    keymap_file.seek(SeekFrom::Start(keymap_size_u64)).unwrap();
    keymap_file.write_all(&[0]).unwrap();
    keymap_file.rewind().unwrap();
    // Memory map the file
    let mut data = unsafe {
        memmap2::MmapOptions::new()
            .map_mut(&keymap_file)
            .expect("Could not access data from memory mapped file")
    };
    // Write the keymap to it
    data[..src.len()].copy_from_slice(src.as_bytes());

    let mut contents = String::new();
    keymap_file.read_to_string(&mut contents).unwrap();

    // Initialize the virtual keyboard with the keymap
    let keymap_raw_fd = keymap_file.into_raw_fd();

    (keymap_raw_fd.clone(), keymap_size_u32)
}

pub fn lookup_sym(c: char) -> Option<xkbcommon::xkb::Keysym> {
    // Special character lookup, otherwise normal lookup
    let keysym = match c {
        '\n' => xkbcommon::xkb::keysyms::KEY_Return,
        '\t' => xkbcommon::xkb::keysyms::KEY_Tab,
        _ => {
            // Convert UTF-8 to a code point first to do the keysym lookup
            let codepoint = format!("U{:X}", c as u32);
            xkbcommon::xkb::keysym_from_name(&codepoint, xkbcommon::xkb::KEYSYM_NO_FLAGS)
        }
    };
    println!("{} {:04X} -> U{:04X}", c, c as u32, keysym);

    // Make sure the keysym is valid
    if keysym != xkbcommon::xkb::keysyms::KEY_NoSymbol {
        Some(keysym)
    } else {
        None
    }
}
