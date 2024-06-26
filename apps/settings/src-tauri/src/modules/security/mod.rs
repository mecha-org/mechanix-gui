pub mod client;
use std::string;

use keyring::Entry;
use users::get_current_username;
use uuid::Uuid;

use crate::constants;
use crate::error::Error;

#[tauri::command]
pub fn set_pin_secret() -> Result<String, Error> {
    println!("Calling::set_pin_secret()");
    match get_current_username() {
        Some(uname) => {
            println!("Running as user with name {:?}", uname);
            let entry = Entry::new(&constants::SECRET_KEY, &(uname.to_string_lossy())).unwrap();

            let secret: Uuid = Uuid::new_v4();
            let _ = entry.set_password(&(secret.to_string()));
            println!("SET SECRET :: {:?}", secret.to_string());
            return Ok(secret.to_string());
        }
        None => {
            println!("The current user does not exist!");
            return Err(Error::Other("The current user does not exist!".to_string()));
        }
    };
}

#[tauri::command]
pub fn get_pin_secret() -> Result<String, Error> {
    println!("Calling::Security::get_pin_secret()");

    match get_current_username() {
        Some(uname) => {
            println!("Running as user with name {:?}", uname);
            let mut entry = Entry::new(&constants::SECRET_KEY, &(uname.to_string_lossy())).unwrap();

            let secret = entry.get_password().unwrap();
            println!("GET SECRET :: {:?}", secret);
            return Ok(secret);
        }
        None => {
            println!("The current user does not exist!");
            return Err(Error::Other("The current user does not exist!".to_string()));
        }
    };
}

#[tauri::command]
pub fn get_security_lock_status() -> Result<bool, Error> {
    println!("Calling::Security::get_security_lock_status");
    match client::Security::is_pin_enabled() {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("ERROR:: {:?} ", e.to_string());
            if e.to_string().contains("Entry not found") {
                return Ok(false);
            }
            return Err(Error::Other(e.to_string()));
        }
    };
}

#[tauri::command]
pub async fn authenticate_pin(pin: String) -> Result<bool, Error> {
    println!("Calling::Security::authenticate_pin() {:?} ", pin);

    let secret = match get_pin_secret() {
        Ok(secret) => secret,
        Err(e) => return Err(e),
    };
    println!("Calling::Security::authenticate_pin()::get_pin_secret {:?} ", secret);

    match client::Security::authenticate(pin, secret).await {
        Ok(v) => {
            println!("authenticate result:  {:?}", v);
            return Ok(v)
        },
        Err(e) => {
            println!("authenticate error:  {:?}", e.to_string());
            return Err(Error::Other(e.to_string()))
        },
    };
}

#[tauri::command]
pub fn change_pin(old_pin: String, new_pin: String, set_new_secret: bool) -> Result<bool, Error> {
    println!("Calling::Security::change_pin()  old pin {:?} and new_pin {:?} ", &old_pin,&new_pin);
   let mut secret = String::from("");

   if set_new_secret {
    secret = match set_pin_secret() {
        Ok(secret) => secret,
        Err(e) => return Err(e),
    };
   }
   else {
    secret = match get_pin_secret() {
        Ok(secret) => secret,
        Err(e) => return Err(e),
    };
   }
  

    println!("change_pin with secret {:?} ", &secret);
    match client::Security::change_password(old_pin, secret, new_pin) {
        Ok(v) => {
            println!("change_password result:  {:?}", v);
            return Ok(v)
        },
        Err(e) => {
            println!("change_password error:  {:?}", e.to_string());
            return Err(Error::Other(e.to_string()))
        },
    };
 
}

#[tauri::command]
pub fn remove_pin_lock(pin: String) -> Result<bool, Error> {
    println!("Calling::Security::remove_pin_lock");
    let mut secret = match get_pin_secret() {
        Ok(secret) => secret,
        Err(e) => return Err(e),
    };
    match client::Security::remove_pin_lock(pin, secret) {
        Ok(v) => return Ok(v),
        Err(e) => {
            println!("ERROR:: {:?} ", e.to_string());
            if e.to_string().contains("Entry not found") {
                return Ok(false);
            }
            return Err(Error::Other(e.to_string()));
        }
    };
}