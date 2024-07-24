use std::collections::HashMap;

use mechanix_store_server::store::StoreEvent;
use tracing::info;
use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.store",
    default_service = "org.mechanix.store",
    default_path = "/org/mechanix/store"
)]
trait Store {
    async fn insert(&self, obj: &str, key_val: (&str, &str)) -> Result<()>;
    async fn insert_batch(&self, obj: &str, keys_vals: HashMap<&str, &str>) -> Result<()>;
    async fn get(&self, obj: &str, key: &str) -> Result<String>;
    async fn get_batch(&self, obj: &str, keys: Vec<&str>) -> Result<HashMap<String, String>>;
    async fn update(&self, obj: &str, key_val: (&str, &str)) -> Result<()>;
    async fn update_batch(&self, obj: &str, keys_vals: HashMap<&str, &str>) -> Result<()>;
    async fn delete(&self, obj: &str, key: &str) -> Result<String>;
    async fn delete_batch(&self, obj: &str, keys: Vec<&str>) -> Result<()>;
    #[zbus(signal)]
    async fn notify(&self, event: StoreEvent) -> Result<()>;
}

pub struct StoreClient;

impl StoreClient {
    pub async fn insert(obj: &str, key_val: (&str, &str)) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.insert(obj, key_val).await?;
        Ok(reply)
    }

    pub async fn insert_batch(obj: &str, keys_vals: HashMap<&str, &str>) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.insert_batch(obj, keys_vals).await?;
        Ok(reply)
    }

    pub async fn get(obj: &str, key: &str) -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.get(obj, key).await?;
        Ok(reply)
    }

    pub async fn get_batch(obj: &str, keys: Vec<&str>) -> Result<HashMap<String, String>> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.get_batch(obj, keys).await?;
        Ok(reply)
    }

    pub async fn update(obj: &str, key_val: (&str, &str)) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.update(obj, key_val).await?;
        Ok(reply)
    }

    pub async fn update_batch(obj: &str, keys_vals: HashMap<&str, &str>) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.update_batch(obj, keys_vals).await?;
        Ok(reply)
    }

    pub async fn delete(obj: &str, key: &str) -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.delete(obj, key).await?;
        Ok(reply)
    }

    pub async fn delete_batch(obj: &str, keys: Vec<&str>) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let reply = proxy.delete_batch(obj, keys).await?;
        Ok(reply)
    }

    pub async fn get_notify() -> Result<NotifyStream<'static>> {
        let connection = Connection::session().await?;
        let proxy = StoreProxy::new(&connection).await?;
        let stream = proxy.receive_notify().await?;
        Ok(stream)
    }
}
