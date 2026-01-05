use anyhow::Result;
use mxsearch::service::MxSearchService;

/// Make sure you have these in your Cargo.toml:
/// zbus = "3"
/// tokio = { version = "1", features = ["full"] }
/// tracing = "0.1"

#[tokio::main]
async fn main() -> Result<()> {
    let mxsearch = match MxSearchService::new().await {
        Ok(mxsearch) => mxsearch,
        Err(e) => {
            eprintln!("Error creating MxSearchService: {}", e);
            return Ok(());
        }
    };

    match mxsearch.search_applications("test").await {
        Ok(applications) => {
            println!("Received values for key '{}':", "test");
            for app in applications {
                println!("  {} = {}", app.name, app.name);
            }
        }
        Err(e) => {
            eprintln!("Error getting setting: {:?}", e);
        }
    }
    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
