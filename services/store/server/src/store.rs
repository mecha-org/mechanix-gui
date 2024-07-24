use std::{collections::HashMap, fmt};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use sled::{open, Batch, Config, Db, Subscriber, Tree};
use tokio::sync::mpsc;
use zbus::zvariant::{self, DeserializeDict, OwnedValue, SerializeDict, Type};

#[derive(Debug, Clone)]
pub enum StoreObj {
    Settings,
    Theme,
    Apps,
}

impl From<&str> for StoreObj {
    fn from(value: &str) -> Self {
        match value {
            "settings" => StoreObj::Settings,
            "theme" => StoreObj::Theme,
            "apps" => StoreObj::Apps,
            _ => StoreObj::Settings,
        }
    }
}

impl fmt::Display for StoreObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreObj::Settings => fmt::Debug::fmt("settings", f),
            StoreObj::Theme => fmt::Debug::fmt("theme", f),
            StoreObj::Apps => fmt::Debug::fmt("apps", f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Type)]
pub enum StoreEvent {
    Insert { keys_vals: HashMap<String, String> },
    Update { keys_vals: HashMap<String, String> },
    Delete { keys_vals: HashMap<String, String> },
}

#[derive(Debug, Clone)]
pub struct Store {
    db: Db,
}

impl Store {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let path = home_dir.join(".config/mechanix/store/db");
        let config = Config::new().path(path);
        let db = config.open().unwrap();
        Self { db }
    }

    fn get_tree(&self, obj: StoreObj) -> Result<Tree> {
        let tree = self.db.open_tree(obj.to_string())?;
        Ok(tree)
    }

    pub fn insert(&mut self, obj: StoreObj, key_val: (&str, &str)) -> Result<()> {
        let tree = self.get_tree(obj)?;
        let _ = tree.insert(key_val.0, key_val.1)?;
        Ok(())
    }

    pub fn insert_batch(&mut self, obj: StoreObj, keys_vals: &HashMap<&str, &str>) -> Result<()> {
        let tree = self.get_tree(obj)?;
        let mut batch = Batch::default();
        for (&key, &val) in keys_vals.into_iter() {
            batch.insert(key, val);
        }
        tree.apply_batch(batch)?;
        Ok(())
    }

    pub fn get(&self, obj: StoreObj, key: &str) -> Result<String> {
        let tree = self.get_tree(obj.clone())?;
        let res = tree.get(key)?;
        if res.is_none() {
            bail!("Key not found");
        }
        let val = std::str::from_utf8(&res.unwrap())?.to_string();
        Ok(val)
    }

    pub fn get_batch<S: Into<String>>(&self, obj: StoreObj, keys: Vec<S>) -> Result<HashMap<S, S>> {
        Ok(HashMap::new())
    }

    pub fn update(&mut self, obj: StoreObj, key_val: (&str, &str)) -> Result<()> {
        let tree = self.get_tree(obj)?;
        let res = tree.insert(key_val.0, key_val.1)?;
        if res.is_none() {
            bail!("Response was none");
        };
        Ok(())
    }

    pub fn update_batch(&self, obj: StoreObj, keys_vals: &HashMap<&str, &str>) -> Result<()> {
        let tree = self.get_tree(obj)?;
        let mut batch = Batch::default();
        for (key, val) in keys_vals.into_iter() {
            batch.insert(*key, *val);
        }
        tree.apply_batch(batch)?;
        Ok(())
    }

    pub fn delete(&self, obj: StoreObj, key: &str) -> Result<String> {
        let tree = self.get_tree(obj)?;
        let res = tree.remove(key)?;
        if res.is_none() {
            bail!("Key not found");
        }
        let val = std::str::from_utf8(&res.unwrap())?.to_string();

        Ok(val)
    }

    pub fn delete_batch(&self, obj: StoreObj, keys: &Vec<&str>) -> Result<()> {
        let tree = self.get_tree(obj)?;
        let mut batch = Batch::default();
        for &key in keys {
            let _ = batch.remove(key);
        }
        let _ = tree.apply_batch(batch)?;
        Ok(())
    }

    pub fn key_exists(&self, obj: StoreObj, key: &str) -> Result<bool> {
        let tree = self.get_tree(obj.clone())?;

        let key_exists_res = tree.contains_key(key);

        if let Err(e) = &key_exists_res {
            bail!("Error while checking if key exists {:?}", e);
        }

        let key_exists = key_exists_res.unwrap();

        if !key_exists {
            bail!("Key does not exist");
        }

        Ok(true)
    }

    pub async fn watch(&self, obj: StoreObj, keys: Vec<&str>, sender: mpsc::Sender<StoreEvent>) {
        let tree = self.get_tree(obj).unwrap();
        let mut subscriber = tree.watch_prefix(vec![]);
        while let Some(event) = (&mut subscriber).await {
            match event {
                sled::Event::Insert { key, value } => {
                    println!("Inserted key: {:?}", key);
                    let key_str = std::str::from_utf8(&key).unwrap().to_string();
                    let val_str = std::str::from_utf8(&value).unwrap().to_string();
                    // let _ = sender
                    //     .send(StoreEvent::Insert {
                    //         key: key_str,
                    //         value: val_str,
                    //     })
                    //     .await;
                }
                sled::Event::Remove { key } => {
                    println!("Inserted key: {:?}", key);
                    let key_str = std::str::from_utf8(&key).unwrap().to_string();
                    // let _ = sender
                    //     .send(StoreEvent::Remove {
                    //         key: key_str,
                    //         value: "".to_string(),
                    //     })
                    //     .await;
                }
            }
        }
    }
}
