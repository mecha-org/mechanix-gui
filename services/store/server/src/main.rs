use std::future;

use anyhow::Result;
mod interfaces;
mod store;
use interfaces::store_interface::StoreInterface;
use store::Store;
use zbus::connection;

#[tokio::main]
async fn main() -> Result<()> {
    let store = Store::new();
    let store_bus = StoreInterface { store };
    let _store_bus_connection = connection::Builder::session()?
        .name("org.mechanix.store")?
        .serve_at("/org/mechanix/store", store_bus.clone())?
        .build()
        .await?;
    let future = future::pending();
    let () = future.await;
    Ok(())
}
