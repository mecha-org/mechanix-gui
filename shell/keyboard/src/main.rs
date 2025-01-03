mod action;
mod components;
mod constants;
mod errors;
mod gui;
mod layout;
mod model;
mod settings;
mod trie;
mod utils;

use action::{Modifier, Modifiers};
use gui::Keyboard;
use mctk_core::{
    context, msg,
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
use mctk_smithay::layer_shell::layer_window::{LayerWindowMessage, LayerWindowParams};
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};
use model::KeyboardModel;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{collections::HashMap, io::SeekFrom};
use std::{
    collections::HashSet,
    io::{Seek, Write},
};
use std::{io::Read, os::fd::IntoRawFd};
use tempfile::tempfile;
use tokio::{
    runtime::Builder,
    sync::mpsc::{self, Receiver},
};
use trie::util::get_trie;
use wayland_protocols_async::{
    zwp_input_method_v2::handler::{
        ContentPurpose, InputMethodEvent, InputMethodHandler, InputMethodMessage,
    },
    zwp_virtual_keyboard_v1::handler::{KeyMotion, VirtualKeyboardHandler, VirtualKeyboardMessage},
};

use crate::gui::Message;
use mctk_core::context::Model;
use settings::{Icons, KeyboardSettings, TrieConfigs};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[derive(Debug)]
enum AppMessage {
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
    ApplyModifiers {
        mods: HashSet<Modifier>,
    },
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
        window_max,
        window_min,
    } = settings.icons.clone();

    svgs.insert("edit-clear-symbolic".to_string(), backspace);

    svgs.insert("key-enter".to_string(), enter);

    svgs.insert("key-shift".to_string(), shift);

    svgs.insert("keyboard-mode-symbolic".to_string(), symbolic);
    svgs.insert("window-max".to_string(), window_max);
    svgs.insert("window-min".to_string(), window_min);

    // create the handler instance
    let layouts = settings.layouts.clone();
    let mut layouts_map: HashMap<ContentPurpose, (i32, u32)> = HashMap::new();
    let default_keymap = load_keymap(layouts.default.clone());
    layouts_map.insert(ContentPurpose::Normal, default_keymap);
    layouts_map.insert(
        ContentPurpose::Terminal,
        load_keymap(layouts.terminal.clone()),
    );
    layouts_map.insert(ContentPurpose::Email, load_keymap(layouts.email.clone()));
    layouts_map.insert(ContentPurpose::Url, load_keymap(layouts.url.clone()));

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.keyboard"));
    let namespace = app_id.clone();

    let mut layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
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
    let (layer_tx, layer_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<Keyboard, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts: layer_shell_opts.clone(),
            svgs,
            layer_tx: Some(layer_tx.clone()),
            layer_rx: Some(layer_rx),
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel.clone()),
        },
    );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let window_tx_3 = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        window_tx_3
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));
    KeyboardModel::get().register_context_handler(context_handler);
    let trie_configs = settings.trie.clone();
    KeyboardModel::get().trie_configs.set(Some(trie_configs));
    KeyboardModel::get().layouts_map.set(layouts_map.clone());
    KeyboardModel::get().window_tx.set(Some(window_tx.clone()));
    KeyboardModel::get().layer_tx.set(Some(layer_tx.clone()));
    KeyboardModel::init();

    // create mpsc channel for interacting with the input_method handler
    let (input_method_msg_tx, input_method_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                // println!("app event {:?}", msg);
                match msg {
                    AppMessage::TextkeyPressed { keycode } => {
                        println!("AppMessage::TextkeyPressed {:?}", keycode);
                        KeyboardModel::key_pressed(keycode);

                        // let virtual_keyboard_msg_tx = virtual_keyboard_msg_tx.clone();
                        // futures::executor::block_on(async move {
                        //     let _ = virtual_keyboard_msg_tx
                        //         .send(VirtualKeyboardMessage::Key {
                        //             keycode: keycode.code - 8,
                        //             keymotion: KeyMotion::Press,
                        //         })
                        //         .await;
                        //     let _ = virtual_keyboard_msg_tx
                        //         .send(VirtualKeyboardMessage::Key {
                        //             keycode: keycode.code - 8,
                        //             keymotion: KeyMotion::Release,
                        //         })
                        //         .await;
                        // });
                    }
                    AppMessage::ApplyModifiers { mods } => {
                        println!("AppMessage::ApplyModifiers {:?}", mods);
                        KeyboardModel::apply_modifiers(mods);

                        // let virtual_keyboard_msg_tx = virtual_keyboard_msg_tx.clone();

                        // let raw_modifiers = mods
                        //     .iter()
                        //     .map(|m| match m {
                        //         Modifier::Control => Modifiers::CONTROL,
                        //         Modifier::Alt => Modifiers::MOD1,
                        //         Modifier::Mod4 => Modifiers::MOD4,
                        //     })
                        //     .fold(Modifiers::empty(), |m, n| m | n);

                        // futures::executor::block_on(async move {
                        //     let _ = virtual_keyboard_msg_tx
                        //         .send(VirtualKeyboardMessage::SetModifiers {
                        //             depressed: raw_modifiers.bits() as u32,
                        //             latched: 0,
                        //             locked: 0,
                        //         })
                        //         .await;
                        // });
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
                }
            }
            calloop::channel::Event::Closed => {}
        };
    });

    loop {
        if app.is_exited {
            break;
        }

        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
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
