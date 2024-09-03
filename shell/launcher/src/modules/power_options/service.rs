use anyhow::Result;
use futures::StreamExt;
use logind::{
    manager::ManagerProxyBlocking,
    session::{SessionProxy, SessionProxyBlocking},
};
use zbus::blocking::Connection;

pub struct PowerOptionsService {}

impl PowerOptionsService {
    pub fn restart() -> Result<()> {
        let connection = Connection::system()?;
        let manager = ManagerProxyBlocking::new(&connection)?;
        let _ = manager.reboot(false)?;
        Ok(())
    }

    pub fn shutdown() -> Result<()> {
        let connection = Connection::system()?;
        let manager = ManagerProxyBlocking::new(&connection)?;
        let _ = manager.power_off(false)?;
        Ok(())
    }

    // pub fn logout() -> Result<()> {
    //     let connection = Connection::system()?;
    //     let manager = ManagerProxyBlocking::new(&connection)?;
    //     let _ = manager.kill_session("6", "all", 1)?;
    //     Ok(())
    // }

    pub fn suspend() -> Result<()> {
        let connection = Connection::system()?;
        let manager = ManagerProxyBlocking::new(&connection)?;
        let _ = manager.suspend(false)?;
        Ok(())
    }

    pub fn lock() -> Result<()> {
        let connection = Connection::system()?;
        let manager = SessionProxyBlocking::new(&connection)?;
        let _ = manager.lock()?;
        Ok(())
    }
}
