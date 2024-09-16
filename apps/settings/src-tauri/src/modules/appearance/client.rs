
use tracing::info;
use zbus::{Connection, proxy , Result};
use anyhow::{bail, Result as AnyhowResult};

#[proxy(
    interface = "org.mechanix.services.Appearance",
    default_service = "org.mechanix.services.Appearance",
    default_path = "/org/mechanix/services/Appearance"
)]
trait AppearanceInterface {
    async fn get_all_wallpapers(&self) -> Result<Vec<String>>;
    async fn get_wallpaper(&self) -> Result<String>;
    async fn set_wallpaper(&self, value: &str) -> Result<()>;
}

pub struct Appearance; 

impl Appearance {

    pub async fn get_all_wallpapers() -> Result<Vec<String>> {
        println!("appearance::client::get_all_wallpapers()");
        let connection = Connection::system().await?;
        let proxy = AppearanceInterfaceProxy::new(&connection).await?;
        let mut reply =  proxy.get_all_wallpapers().await?;
        println!("get_all_wallpapers reply ====> {:?}", reply);
        Ok(reply)
    }

    pub async fn get_wallpaper() -> Result<String> {
        println!("appearance::client::get_wallpaper()");
        let connection = Connection::system().await?;
        let proxy = AppearanceInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_wallpaper().await?;
        Ok(reply)
    }

    pub async fn set_wallpaper(value: &str) -> Result<()> {
        println!("appearance::client::set_wallpaper()");
        let connection = Connection::system().await?;
        let proxy = AppearanceInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_wallpaper(value).await?;
        Ok(())
    }
}