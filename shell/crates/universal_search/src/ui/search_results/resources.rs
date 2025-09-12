use bevy::prelude::*;

use crate::types::{DesktopApp, SearchResult};

#[derive(Debug, Resource, Clone)]
pub struct SearchResults(pub Vec<SearchResult>);

#[derive(Debug, Resource, Clone)]
pub struct BrowserApps(pub Vec<DesktopApp>);
