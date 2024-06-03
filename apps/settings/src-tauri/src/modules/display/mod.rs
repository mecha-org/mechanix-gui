pub mod client;
use crate::error::Error;

#[tauri::command]
pub async fn get_brightness() -> Result<(u8), Error> {
    println!("get_brightness....");
    match client::Display::get_brightness_percentage().await {
        Ok(v) => {
            println!("get_brightness result: {:?} ", v.to_owned());
            return Ok(v)
        },
        Err(e) => {
            println!("get_brightness error: {:?} ", e.to_owned());
        
            return Err(Error::Other(e.to_string()))
        }
    };
}


#[tauri::command]
pub async fn set_brightness(value: u8) -> Result<(), Error> {
    println!("set_brightness....{:?}", value.to_owned());
    match client::Display::set_brightness_percentage(value).await {
        Ok(v) => {
            println!("set_brightness result: {:?} ", v.to_owned());
            return Ok(v)
        },
        Err(e) => {
            println!("set_brightness error: {:?} ", e.to_owned());
        
            return Err(Error::Other(e.to_string()))
        }
    };
}