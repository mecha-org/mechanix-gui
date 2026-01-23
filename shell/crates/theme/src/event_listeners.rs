use dispatcher::Dispatcher;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::*;
use gpui::QuitMode::Default;
use crate::{ThemeEvents, helpers::parse_oklcha_str, manager::ThemeManager, prelude::*};

pub fn listen_dispatcher(cx: &mut App, mut theme_tx: mpsc::Sender<ThemeEvents>) {
    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    _ = cx
        .background_executor()
        .spawn(async move {
            while let Ok(msg) = dispatcher_rx.recv().await {
                match msg {
                    dispatcher::Message::SetThemeMode(mode) => {
                        let _ = theme_tx.send(ThemeEvents::SetThemeMode(mode)).await;
                    }
                    dispatcher::Message::SetThemeColors {
                        accent,
                    } => {
                        let _ = theme_tx
                            .send(ThemeEvents::SetThemeColors {
                                accent
                            })
                            .await;
                    }
                    dispatcher::Message::SetPrimaryFont(font) => {
                        let _ = theme_tx.send(ThemeEvents::SetPrimaryFont(font)).await;
                    }
                    dispatcher::Message::SetSecondaryFont(font) => {
                        let _ = theme_tx.send(ThemeEvents::SetSecondaryFont(font)).await;
                    }
                    dispatcher::Message::SetTertiaryFont(font) => {
                        let _ = theme_tx.send(ThemeEvents::SetTertiaryFont(font)).await;
                    }
                    _ => (),
                }
            }
        })
        .detach();
}

pub fn listen_theme_channel(cx: &mut App, mut theme_rx: mpsc::Receiver<ThemeEvents>) {
    cx.spawn(async move |app| {
        while let Some(msg) = theme_rx.next().await {
            match msg {
                ThemeEvents::SetThemeColors {
                    accent,
                } => {
                    _ = app.update(|mut cx| {
                        let mut colors = ThemeManager::global(cx).colors.clone();
                        colors.accent_color = parse_oklcha_str(&accent).unwrap();
                        ThemeManager::global_mut(cx).set_colors(colors);
                        ThemeManager::apply(cx);
                        cx.refresh_windows();
                    });
                }
                ThemeEvents::SetThemeMode(mode) => {
                    _ = app.update(|cx| {
                        ThemeManager::global_mut(cx).set_mode(ThemeMode::from_str(&mode));
                        ThemeManager::apply(cx);
                        cx.refresh_windows();
                    });
                }
                ThemeEvents::SetPrimaryFont(font) => {
                    _ = app.update(|cx| {
                        Fonts::global_mut(cx).primary = font.into();
                        cx.refresh_windows();
                    })
                }
                ThemeEvents::SetSecondaryFont(font) => {
                    _ = app.update(|cx| {
                        Fonts::global_mut(cx).secondary = font.into();
                        cx.refresh_windows();
                    })
                }
                ThemeEvents::SetTertiaryFont(font) => {
                    _ = app.update(|cx| {
                        Fonts::global_mut(cx).tertiary = font.into();
                        cx.refresh_windows();
                    })
                }
            }
        }
    })
    .detach();
}
