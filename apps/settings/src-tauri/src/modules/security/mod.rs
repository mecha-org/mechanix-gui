pub mod client;
use crate::error::Error;

#[tauri::command]
pub async fn get_security_lock_status() -> Result<bool, Error> {
    println!("security::mod::get_security_lock_status");
    match client::Security::is_pin_enabled().await {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("ERROR:: {:?} ", e.to_string());
            if e.to_string().contains("Entry not found") {
                return Ok(false);
            }
            return Err(Error::Other(e.to_string()))
        }
    };
}