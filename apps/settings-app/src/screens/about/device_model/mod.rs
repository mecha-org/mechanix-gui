pub mod identity_manager;
pub mod settings_manager;

use identity_manager::IdentityClient;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref DEVICE_MODEL: DeviceModel = DeviceModel {
        is_provisioned: Context::new(false)
    };
}

#[derive(Model)]
pub struct DeviceModel {
    pub is_provisioned: Context<bool>,
}

impl DeviceModel {
    pub fn get() -> &'static Self {
        &DEVICE_MODEL
    }

    pub fn update() {
        RUNTIME.spawn(async {
            let mut service_client = IdentityClient::new().await.unwrap();

            let provision_status = service_client.get_machine_provision_status().await.unwrap();

            // println!("Battery provision_status: {}", provision_status);
            DeviceModel::get()
                .is_provisioned
                .set(provision_status.status);
        });
    }
}

// get_machine_provision_status - if device is provisioned

// // let machine_id_data : any = await invoke('get_machine_id');
// IdentityClient - getting_machine_id

// let machine_name_data : any = await invoke('get_machine_info', {key: "identity.machine.name"});
// let machine_icon_data : any = await invoke('get_machine_info', {key: "identity.machine.icon_url"});

// async fn get_machine_info(key: String) -> Result<GetSettingsResponse, Error> {
//     let request = SettingsClient::new().await;
//     let mut service_client = match request {
//         Ok(r) => r,
//         Err(e) => {
//             println!(
//                 "get_machine_info::service_client::new::error::  {:?} ",
//                 &e.to_string()
//             );
//             return Err(Error::Io(e.into()));
//         }
//     };

//     let response: GetSettingsResponse = match service_client.get_settings_data(key.clone()).await {
//         Ok(response) => response.into(),
//         Err(e) => {
//             println!(
//                 "get_machine_info::service_client::get_settings_data::error::  {:?} ",
//                 &e.to_string()
//             );
//             return Err(Error::Io(e.into()));
//         }
//     };
