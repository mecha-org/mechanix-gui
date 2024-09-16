pub mod client;
use crate::error::Error;

#[tauri::command]
pub async fn get_available_wallpapers() -> Result<Vec<String>, Error> {
    println!("appearance::mod::get_available_wallpapers");
    match client::Appearance::get_all_wallpapers().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn get_applied_wallpaper() -> Result<String, Error> {
    println!("appearance::mod::get_applied_wallpaper");
    match client::Appearance::get_wallpaper().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn set_wallpaper(value: String) -> Result<(), Error> {
    println!("appearance::mod::set_wallpaper  {:?}", value);
    match client::Appearance::set_wallpaper(&value).await {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("appearance::mod::set_wallpaper::error() {:?}", e);
            return Err(Error::Other(e.to_string()))
        }
    };
}