//! Basic example: Get Battery Percentage using freedesktop-upower-client

use freedesktop_upower_client::service::UpowerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let upower_service = UpowerService::new().await?;
    // Create a channel to receive the battery percentage result
    let battery_percentage = match upower_service.get_percentage().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to get battery percentage: {e}");
            return Err(e.into());
        }
    };
    println!("Battery percentage: {}", battery_percentage);


    Ok(())
}
