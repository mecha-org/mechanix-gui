
use tracing::info;
use zbus::{Connection, proxy , Result,};


#[proxy(
    interface = "org.mechanix.services.Display",
    default_service = "org.mechanix.services.Display",
    default_path = "/org/mechanix/services/Display"
)]
trait DisplayBusInterface {
    async fn get_brightness(&self) -> Result<u8>;
    // async fn get_screen_timeout(&self) -> Result<u32>;
    // async fn set_screen_timeout(&self, value: u32) ->Result<u32>;

    


}

pub struct Display; 

impl Display {
    pub async fn get_brightness_percentage() -> Result<u8> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply =  proxy.get_brightness().await?;
        println!("get_brightness_percentage reply: {:?}", reply);
        Ok(reply)
    }

    // pub async fn get_screen_timeout() -> Result<String> {
    //     let connection = Connection::system().await?;
    //     let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    //     let reply =  proxy.get_screen_timeout().await?;
    //     info!("get_screen_timeout reply: {:?}", reply);
    //     let result = format!("{}s", reply);
    //     Ok(result)
    // }

    // pub async fn set_screen_timeout(value: u32) -> Result<String> {
    //     let connection = Connection::system().await?;
    //     let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    //     let reply = match proxy.set_screen_timeout(value).await {
    //         Ok(value)=> value,
    //         Err(e)=>{
    //             print!("error {:?}",e);
    //             0
    //         }
    //     };
    //     let result = format!("{}s", reply);
    //     Ok(result)
    // }

}
