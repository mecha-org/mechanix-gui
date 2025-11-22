use crate::ui::desktop_entries;

use super::desktop_entries::DesktopEntry;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use std::sync::{ Arc, RwLock };

pub struct SimpleContext<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> SimpleContext<T> {
    pub fn new(value: T) -> Self {
        SimpleContext {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    pub fn get(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }
}

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref DesktopEntries: DesktopEntriesModel = DesktopEntriesModel {
        entries: SimpleContext::new(desktop_entries::DesktopEntries::all().unwrap()),
    };
}

pub struct DesktopEntriesModel {
    pub entries: SimpleContext<Vec<DesktopEntry>>,
}

impl DesktopEntriesModel {
    pub fn get() -> &'static Self {
        &DesktopEntries
    }
}
