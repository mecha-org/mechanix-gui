use dispatcher::Dispatcher;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::*;

use crate::{ThemeEvents, helpers::parse_oklcha_str, manager::ThemeManager, prelude::*};

pub fn listen_dispatcher(cx: &mut App, mut theme_tx: mpsc::Sender<ThemeEvents>) {
    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    _ = cx
        .background_executor()
        .spawn(async move {
            while let Ok(msg) = dispatcher_rx.try_recv() {
                match msg {
                    dispatcher::Message::SetThemeMode(mode) => {
                        let _ = theme_tx.send(ThemeEvents::SetThemeMode(mode)).await;
                    }
                    dispatcher::Message::SetThemeColors {
                        accent,
                        background,
                        foreground,
                    } => {
                        let _ = theme_tx
                            .send(ThemeEvents::SetThemeColors {
                                accent,
                                background,
                                foreground,
                            })
                            .await;
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
                    background,
                    foreground,
                } => {
                    _ = app.update(|cx| {
                        ThemeManager::global_mut(cx).set_colors(ColorsSetting {
                            accent_color: parse_oklcha_str(&accent).unwrap(),
                            background_color: parse_oklcha_str(&background).unwrap(),
                            foreground_color: parse_oklcha_str(&foreground).unwrap(),
                        });
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
            }
        }
    })
    .detach();
}
