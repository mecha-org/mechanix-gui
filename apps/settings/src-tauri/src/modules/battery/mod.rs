pub mod client;
use crate::error::Error;

#[tauri::command]
pub async fn get_battery_percentage() -> Result<f64, Error> {
    println!("battery::mod::get_battery_percentage");
    match client::Power::get_battery_percentage().await{
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    }
}

#[tauri::command]
pub async fn get_avilable_performance_modes() -> Result<Vec<String>, Error> {
    println!("battery::mod::get_performance_modes");
    match client::Power::get_all_performance_modes().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn get_current_performance_mode() -> Result<String, Error> {
    println!("battery::mod::get_current_performance_mode");
    match client::Power::get_current_performance_mode().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}


#[tauri::command]
pub async fn set_performance_mode(value: String) -> Result<(), Error> {
    println!("battery::mod::set_performance_mode  {:?}", value);
    match client::Power::set_cpu_governor(&value).await {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("battery::mod::set_performance_mode::error() {:?}", e);
            return Err(Error::Other(e.to_string()))
        }
    };
}
