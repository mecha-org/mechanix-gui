use tokio;
use extensions::interface::ExtensionService;
#[tokio::main]
async fn main()
{
 ExtensionService::start_service().await;
}