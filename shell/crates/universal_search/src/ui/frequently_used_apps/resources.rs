use bevy::prelude::*;

use crate::types::DesktopApp;

#[derive(Debug, Resource, Clone)]
pub struct FrequentlyUsedApps(pub Vec<DesktopApp>);
