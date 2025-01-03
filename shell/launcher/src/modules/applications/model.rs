use desktop_entries::DesktopEntry;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref DesktopEntries: DesktopEntriesModel = DesktopEntriesModel {
        entries: Context::new(desktop_entries::DesktopEntries::all().unwrap()),
        is_running: Context::new(false)
    };
}

#[derive(Model)]
pub struct DesktopEntriesModel {
    pub entries: Context<Vec<DesktopEntry>>,
    is_running: Context<bool>,
}

impl DesktopEntriesModel {
    pub fn get() -> &'static Self {
        &DesktopEntries
    }

    // pub fn entries() -> Vec<DesktopEntry> {
    //     (*Self::get().entries.get()).to_vec()
    // }

    fn run_desktop_entries_watch_stream() {
        RUNTIME.spawn(async move { Self::watch_desktop_entries() });
    }

    fn watch_desktop_entries() {
        RUNTIME.spawn(async move {
            let (mut watcher, mut rx) = Self::async_watcher().unwrap();
            let dirs = desktop_entries::DesktopEntries::get_dirs().unwrap();

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            for dir in dirs {
                // println!("watching {:?}", dir.join("applications"));
                let _ = watcher.watch(dir.join("applications").as_ref(), RecursiveMode::Recursive);
            }

            while let Some(res) = rx.next().await {
                match res {
                    Ok(event) => {
                        if let Some(path) = event.paths.get(0) {
                            if path.extension().and_then(OsStr::to_str) != Some("desktop") {
                                continue;
                            }
                        } else {
                            continue;
                        }

                        // println!("file event is {:?}", event);

                        match event.kind {
                            notify::EventKind::Create(create_kind) => match create_kind {
                                notify::event::CreateKind::File => {
                                    println!("file created: {:?}", event);
                                    let path = event.paths.get(0).unwrap();
                                    let entry =
                                        desktop_entries::DesktopEntries::from_path(path).unwrap();
                                    let mut entries = Self::get().entries.get().to_vec();
                                    entries.push(entry);
                                    Self::get().entries.set(entries);
                                }
                                _ => (),
                            },
                            notify::EventKind::Modify(modify_kind) => match modify_kind {
                                notify::event::ModifyKind::Name(rename_mode) => {
                                    println!("file modified: {:?}", event);
                                    match rename_mode {
                                        notify::event::RenameMode::To => {
                                            let path = event.paths.get(0).unwrap();
                                            if path.extension().and_then(OsStr::to_str)
                                                != Some("desktop")
                                            {
                                                continue;
                                            }
                                            let entry =
                                                desktop_entries::DesktopEntries::from_path(path)
                                                    .unwrap();
                                            let mut entries = Self::get().entries.get().to_vec();
                                            entries.push(entry);
                                            Self::get().entries.set(entries);
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            },
                            notify::EventKind::Remove(remove_kind) => match remove_kind {
                                notify::event::RemoveKind::File => {
                                    println!("file removed: {:?}", event);
                                    let path = event.paths.get(0).unwrap();
                                    let app_id = path
                                        .file_stem()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or_default();
                                    let mut entries = Self::get().entries.get().to_vec();
                                    entries.retain(|e| e.app_id != app_id);
                                    Self::get().entries.set(entries);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
    }

    fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let watcher = RecommendedWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            },
            Config::default(),
        )?;

        Ok((watcher, rx))
    }

    pub fn run() {
        if *DesktopEntriesModel::get().is_running.get() {
            return;
        }

        DesktopEntriesModel::get().is_running.set(true);
        Self::run_desktop_entries_watch_stream();
    }
}
