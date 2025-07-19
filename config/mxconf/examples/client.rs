use mxconf::cli::watch_setting;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Example: Watch for changes to a key
    println!("Example 1: Watch for changes to a key");
    println!("Press Ctrl+C to stop watching");
    watch_setting("org.mechanix.keyboard", &Some("general.enabled".to_string())).await?;

    // Note: The following examples won't run because the watch_setting function
    // blocks indefinitely. In a real application, you would use these functions
    // based on user input or in separate tasks.

    // Example: Get a setting
    // let value = get_setting("org.mechanix.keyboard.general.enabled").await?;
    // println!("Value: {}", value);

    // Example: Set a setting
    // let result = set_setting("org.mechanix.keyboard.general.enabled", "true").await?;
    // println!("Result: {}", result);

    // Example: List all schemas
    // list_schemas().await?;

    Ok(())
}
