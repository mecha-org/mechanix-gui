use anyhow::Result;
use std::collections::HashMap;
use tracing::{debug, info};
use zwlr_foreign_toplevel_v1_async::handler::ToplevelKey;

#[derive(Debug, Clone)]
pub struct ToplevelService {
    pub top_levels_by_app_id: HashMap<String, HashMap<ToplevelKey, ()>>,
}

impl ToplevelService {
    pub fn new() -> Self {
        Self {
            top_levels_by_app_id: HashMap::new(),
        }
    }

    pub fn top_levels_by_app_id_exists(&self, app_id: String) -> Result<bool> {
        Ok(self.top_levels_by_app_id.contains_key(&app_id))
    }

    pub fn get_top_levels_keys_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Option<HashMap<ToplevelKey, ()>>> {
        let top_level_keys = match self.top_levels_by_app_id.get_key_value(&app_id) {
            Some((k, v)) => Some(v.clone()),
            None => None,
        };

        Ok(top_level_keys)
    }

    pub fn add_top_levels_by_app_id(&mut self, app_id: String, key: ToplevelKey) -> Result<bool> {
        if !(app_id.len() > 0) {
            return Ok(false);
        }

        let mut top_level_keys: HashMap<ToplevelKey, ()> =
            match self.top_levels_by_app_id.get_key_value(&app_id) {
                Some((k, v)) => v.clone(),
                None => HashMap::new(),
            };
        top_level_keys.insert(key, ());
        self.top_levels_by_app_id.insert(app_id, top_level_keys);
        Ok(true)
    }

    pub fn get_all_top_levels(&self) -> Result<HashMap<String, HashMap<ToplevelKey, ()>>> {
        Ok(self.top_levels_by_app_id.clone())
    }

    pub fn remove_top_level_by_key(&mut self, key: ToplevelKey) -> Result<bool> {
        debug!("all top levels are {:?}", self.top_levels_by_app_id);

        let top_level_op = self
            .top_levels_by_app_id
            .clone()
            .into_iter()
            .find(|(_, value)| value.contains_key(&key));

        match top_level_op {
            Some((top_level_key, mut top_level_value)) => {
                top_level_value.remove_entry(&key);

                if top_level_value.is_empty() {
                    self.top_levels_by_app_id.remove_entry(&top_level_key);
                } else {
                    self.top_levels_by_app_id
                        .insert(top_level_key, top_level_value);
                }
            }
            None => (),
        }

        Ok(true)
    }
}
