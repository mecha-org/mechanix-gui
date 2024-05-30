use anyhow::Error;
use keyring::Entry;
use sha256::digest;
use zbus::{fdo::Error as ZbusError, interface};

pub struct SecurityBusInterface {}

#[interface(name = "org.mechanix.services.Security")]
impl SecurityBusInterface {
    pub async fn set_pin_lock(&self, pin: String) -> Result<bool, ZbusError> {
        self.set_pin(pin).await
    }
    pub async fn remove_pin_lock(&self) -> Result<bool, ZbusError> {
        self.remove_pin().await
    }
    pub async fn is_pin_lock_enabled(&self) -> Result<bool, ZbusError> {
        self.pin_enabled().await
    }
    pub async fn authenticate_pin(&self, pin: String) -> Result<bool, ZbusError> {
        self.authenticate(pin).await
    }
}

impl SecurityBusInterface {
    pub async fn get_entry(&self) -> anyhow::Result<Entry> {
        let entry = Entry::new("mechanix-desktop", "mecha")?;
        Ok(entry)
    }

    pub async fn get_password(&self) -> anyhow::Result<String> {
        let entry = self.get_entry().await?;
        let password = entry.get_password()?;
        Ok(password)
    }

    pub async fn set_pin(&self, pin: String) -> Result<bool, ZbusError> {
        if pin.len() != 4 {
            return Err(ZbusError::Failed("Pin length not equal to 4".to_string()));
        }

        let entry_r = self.get_entry().await;
        if let Err(e) = &entry_r {
            println!("Error while creating entry {:?}", e);
            return Err(ZbusError::Failed("Setup pin failed".to_string()));
        }

        let entry = entry_r.unwrap();
        let encoded_pin = digest(pin);
        let password_r = entry.set_password(&encoded_pin);

        if let Err(e) = &password_r {
            println!("error while setting entry password {:?}", e);
            return Err(ZbusError::Failed("Setup pin failed".to_string()));
        }

        Ok(true)
    }

    pub async fn remove_pin(&self) -> Result<bool, ZbusError> {
        let entry_r = self.get_entry().await;
        if let Err(e) = &entry_r {
            println!("Error while creating entry {:?}", e);
            return Err(ZbusError::Failed("Remove pin failed".to_string()));
        }

        let entry = entry_r.unwrap();

        let password_r = entry.delete_password();

        if let Err(e) = &password_r {
            println!("error while removing entry password {:?}", e);
            return Err(ZbusError::Failed("Remove pin failed".to_string()));
        }

        Ok(true)
    }

    pub async fn pin_enabled(&self) -> Result<bool, ZbusError> {
        let entry_r = self.get_entry().await;
        if let Err(e) = &entry_r {
            println!("Error while creating entry {:?}", e);
            return Err(ZbusError::Failed("Pin enabled check failed".to_string()));
        }

        let entry = entry_r.unwrap();

        let password_r = entry.get_password();

        if let Err(e) = &password_r {
            println!("error while getting entry password {:?}", e);
            let error_message = match e {
                keyring::Error::NoEntry => "Entry not found",
                _ => "Pin enabled check failed",
            };

            return Err(ZbusError::Failed(error_message.to_string()));
        }

        let password = password_r.unwrap();
        if password.is_empty() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn authenticate(&self, pin: String) -> Result<bool, ZbusError> {
        let entry_r = self.get_entry().await;
        if let Err(e) = &entry_r {
            println!("Error while creating entry {:?}", e);
            return Err(ZbusError::Failed("Remove pin failed".to_string()));
        }

        let entry = entry_r.unwrap();

        let password_r = entry.get_password();
        if let Err(e) = &password_r {
            println!("error while getting entry password {:?}", e);
            let error_message = match e {
                keyring::Error::NoEntry => "Entry not found",
                _ => "Pin enabled check failed",
            };

            return Err(ZbusError::Failed(error_message.to_string()));
        }

        let password = password_r.unwrap();
        if password.is_empty() {
            return Ok(false);
        }

        //calculate sha256 of pin
        let encoded_pin = digest(pin);

        if !(password == encoded_pin) {
            return Err(ZbusError::AuthFailed("Authentication Failed".to_string()));
        }

        Ok(true)
    }
}
