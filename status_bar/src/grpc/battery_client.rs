use tonic::transport::Channel;

#[allow(non_snake_case)]
pub mod battery {
    tonic::include_proto!("battery");
}

pub use battery::{
    power_supply_service_client::PowerSupplyServiceClient,
    power_supply_service_server::{PowerSupplyService, PowerSupplyServiceServer},
    Empty,
};

use self::battery::GetPowerSupplyInfoResponse;

pub struct BatteryManagerClient {
    client: PowerSupplyServiceClient<Channel>,
}

impl BatteryManagerClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = PowerSupplyServiceClient::connect(url).await?;

        Ok(Self { client })
    }

    pub async fn get_battery_status(
        &mut self,
    ) -> Result<GetPowerSupplyInfoResponse, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_power_supply_info(request).await?;
        println!("bluetooth response is {:?}", response);
        Ok(response.into_inner())
    }
}
