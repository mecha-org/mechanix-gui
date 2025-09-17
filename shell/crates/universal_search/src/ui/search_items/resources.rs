use bevy::prelude::*;

use crate::types::SearchResult;

#[derive(Debug, Resource, Clone)]
pub struct SearchItems(pub Vec<SearchResult>);
