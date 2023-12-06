use tonic::transport::Channel;

#[derive(Debug, Default)]
pub struct Bluetooth {}

#[allow(non_snake_case)]
pub mod bluetooth {
    tonic::include_proto!("bluetooth");
}

pub use bluetooth::{
    bluetooth_service_client::BluetoothServiceClient,
    bluetooth_service_server::{BluetoothService, BluetoothServiceServer},
    BluetoothStatus, Empty, EmptyResponse,
};

pub struct BluetoothManagerClient {
    client: BluetoothServiceClient<Channel>,
}

impl BluetoothManagerClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = BluetoothServiceClient::connect(url).await?;

        Ok(Self { client })
    }

    pub async fn get_bluetooth_status(
        &mut self,
    ) -> Result<BluetoothStatus, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_bluetooth_status(request).await?;
        println!("bluetooth response is {:?}", response);
        Ok(response.into_inner())
    }
}
