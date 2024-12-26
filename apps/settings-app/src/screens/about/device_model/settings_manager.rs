use anyhow::{bail, Result};
use tonic::transport::Channel;

#[allow(non_snake_case)]
pub mod settings {
    tonic::include_proto!("settings");
}

pub use settings::{
    settings_service_client::SettingsServiceClient, GetSettingsRequest, GetSettingsResponse,
};

#[derive(Debug, Clone)]
pub struct SettingsClient {
    client: SettingsServiceClient<Channel>,
}
impl SettingsClient {
    pub async fn new() -> Result<Self> {
        let url = "http://localhost:3001".to_string();

        let client: SettingsServiceClient<Channel> = match SettingsServiceClient::connect(url).await
        {
            Ok(client) => client,
            Err(e) => {
                bail!(e);
            }
        };

        Ok(Self { client })
    }

    pub async fn get_settings_data(&mut self, key: String) -> Result<GetSettingsResponse> {
        let request = tonic::Request::new(GetSettingsRequest { key: key });

        let response = match self.client.get(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                bail!(e);
            }
        };
        Ok(response)
    }
}
