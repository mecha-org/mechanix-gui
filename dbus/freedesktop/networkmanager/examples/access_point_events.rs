use futures::StreamExt;
use networkmanager::service::NetworkManagerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let network_manager = NetworkManagerService::new().await?;
    let mut receiver = network_manager.stream_access_point_events().await;

    let handler = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(new_access_point) => {
                    println!("access_point event: {:?}", new_access_point);
                }
                Err(e) => {
                    eprintln!("Error getting wifi state: {e}");
                }
            }
        }
    });

    handler.await.unwrap();
    Ok(())
}
