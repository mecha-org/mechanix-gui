use std::collections::HashMap;
use zbus::{fdo::Error as ZbusError, interface, zvariant::Type, SignalContext};

use crate::store::{Store, StoreEvent, StoreObj};

#[derive(Clone)]
pub struct StoreInterface {
    pub store: Store,
}

#[interface(name = "org.mechanix.store")]
impl StoreInterface {
    pub async fn insert(
        &mut self,
        obj: &str,
        key_val: (&str, &str),
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<bool, ZbusError> {
        let res = self.store.insert(StoreObj::from(obj), key_val);
        if let Err(e) = &res {
            println!("Error while inserting in store {:?}", e);
            return Err(create_err("Failed to insert"));
        }

        let mut keys_vals = HashMap::new();
        keys_vals.insert(key_val.0.to_string(), key_val.1.to_string());
        let _ = self.notify(&ctxt, StoreEvent::Insert { keys_vals }).await;
        Ok(true)
    }

    pub async fn insert_batch(
        &mut self,
        obj: &str,
        keys_vals: HashMap<&str, &str>,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<bool, ZbusError> {
        let res = self.store.insert_batch(StoreObj::from(obj), &keys_vals);
        if let Err(e) = &res {
            println!("Error while batch inserting in store {:?}", e);
            return Err(create_err("Failed to insert batch"));
        }

        let _ = self
            .notify(
                &ctxt,
                StoreEvent::Insert {
                    keys_vals: keys_vals
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                },
            )
            .await;

        Ok(true)
    }

    pub async fn get(&self, obj: &str, key: &str) -> Result<String, ZbusError> {
        let res = self.store.get(StoreObj::from(obj), key);
        if let Err(e) = &res {
            println!("Error while getting in store {:?}", e);
            return Err(create_err("Failed to get"));
        }

        Ok(res.unwrap())
    }
    pub async fn get_batch(
        &self,
        obj: &str,
        keys: Vec<String>,
    ) -> Result<HashMap<String, String>, ZbusError> {
        let key_vals = self.store.get_batch(StoreObj::from(obj), keys).unwrap();
        Ok(key_vals)
    }

    pub async fn update(
        &mut self,
        obj: &str,
        key_val: (&str, &str),
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let res = self.store.update(StoreObj::from(obj), key_val);
        if let Err(e) = &res {
            println!("Error while updating in store {:?}", e);
            return Err(create_err("Failed to update"));
        }

        let mut keys_vals = HashMap::new();
        keys_vals.insert(key_val.0.to_string(), key_val.1.to_string());
        let _ = self.notify(&ctxt, StoreEvent::Update { keys_vals }).await;

        Ok(())
    }

    pub async fn update_batch(
        &mut self,
        obj: &str,
        keys_vals: HashMap<&str, &str>,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let res = self.store.update_batch(StoreObj::from(obj), &keys_vals);
        if let Err(e) = &res {
            println!("Error while batch updating in store {:?}", e);
            return Err(create_err("Failed to update batch"));
        }

        let _ = self
            .notify(
                &ctxt,
                StoreEvent::Update {
                    keys_vals: keys_vals
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                },
            )
            .await;

        Ok(())
    }

    pub async fn delete(
        &self,
        obj: &str,
        key: &str,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<String, ZbusError> {
        let res = self.store.delete(StoreObj::from(obj), key);
        if let Err(e) = &res {
            println!("Error while deleting in store {:?}", e);
            return Err(create_err("Failed to delete"));
        }

        let mut keys_vals = HashMap::new();
        keys_vals.insert(key.to_string(), "".to_string());
        let _ = self.notify(&ctxt, StoreEvent::Delete { keys_vals }).await;

        Ok(res.unwrap())
    }

    pub async fn delete_batch(
        &mut self,
        obj: &str,
        keys: Vec<&str>,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let res = self.store.delete_batch(StoreObj::from(obj), &keys);
        if let Err(e) = &res {
            println!("Error while batch deleting in store {:?}", e);
            return Err(create_err("Failed to delete batch"));
        }

        let _ = self
            .notify(
                &ctxt,
                StoreEvent::Update {
                    keys_vals: keys
                        .iter()
                        .map(|k| (k.to_string(), "".to_string()))
                        .collect(),
                },
            )
            .await;

        Ok(())
    }

    #[zbus(signal)]
    async fn notify(&self, ctxt: &SignalContext<'_>, event: StoreEvent) -> Result<(), zbus::Error>;
}

fn create_err<S: Into<String>>(msg: S) -> ZbusError {
    return ZbusError::Failed(msg.into());
}
