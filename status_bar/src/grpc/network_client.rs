use tonic::transport::Channel;

#[allow(non_snake_case)]
pub mod networkmanager {
    tonic::include_proto!("networkmanager");
}

pub use networkmanager::{
    network_manager_service_client::NetworkManagerServiceClient, Empty, RemoveNetworkRequest,
    WifiConnectRequest,
};

use self::networkmanager::{NetworkResult, ScanResult, WifiStatusResponse};

pub struct NetworkManagerClient {
    client: NetworkManagerServiceClient<Channel>,
}

pub struct ExtendedWifiStatusResponse {
    pub wifi_on: bool,
    pub current_network: Option<ScanResult>,
}

impl NetworkManagerClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = NetworkManagerServiceClient::connect(url).await?;

        Ok(Self { client })
    }

    pub async fn scan_wireless_network(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.scan_wireless_network(request).await?;

        println!("Scan Results: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn connect_wireless_network(
        &mut self,
        ssid: &str,
        psk: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(WifiConnectRequest {
            ssid: ssid.to_string(),
            psk: psk.to_string(),
        });
        let response = self.client.connect_wireless_network(request).await?;

        println!("Connect Response: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn disconnect_wireless_network(
        &mut self,
        network_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(RemoveNetworkRequest { network_id });
        let response = self.client.disconnect_wireless_network(request).await?;

        println!("Disconnect Response: {:?}", response.into_inner());

        Ok(())
    }

    pub async fn get_wireless_network_status(
        &mut self,
    ) -> Result<(ExtendedWifiStatusResponse), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_wifi_status(request).await?;
        //println!("Network Status Response: {:?}", response.into_inner());
        let wifi_status = response.into_inner();
        let request2 = tonic::Request::new(Empty {});
        let mut current_network: Option<ScanResult> = match wifi_status.wifi_on {
            true => {
                let response2 = self.client.get_current_network(request2).await;
                let current_network = match response2 {
                    Ok(r) => Option::from(r.into_inner()),
                    Err(_) => None,
                };
                current_network
            }
            false => None,
        };

        Ok((ExtendedWifiStatusResponse {
            wifi_on: wifi_status.wifi_on,
            current_network: current_network,
        }))
    }

    pub async fn get_current_wireless_network(
        &mut self,
    ) -> Result<(ScanResult), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_current_network(request).await?;
        //println!("Network Status Response: {:?}", response.into_inner());

        Ok((response.into_inner()))
    }
}
