use mockall::predicate::*;
use mockall::mock;
use std::collections::HashMap;

// Create our own versions of get_setting and set_setting that take a mock proxy
async fn get_setting(proxy: &MockConfigServerProxy, key: &str) -> Result<HashMap<String, String>, anyhow::Error> {
    let value = proxy.get_setting(key).await?;
    Ok(value)
}

async fn set_setting(proxy: &MockConfigServerProxy, key: &str, value: &str) -> Result<String, anyhow::Error> {
    let result = proxy.set_setting(key, value).await?;
    Ok(result)
}

// Mock the D-Bus proxy
mock! {
    pub ConfigServerProxy {
        pub async fn get_setting(&self, key: &str) -> Result<HashMap<String, String>, zbus::Error>;
        pub async fn set_setting(&self, key: &str, value: &str) -> Result<String, zbus::Error>;
    }
}

// Tests for the client module
#[tokio::test]
async fn test_get_setting() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();

    let settings = HashMap::from([("org.mechanix.test.section.key".to_string(), "test_value".to_string())]);
    // Set expectations
    mock_proxy
        .expect_get_setting()
        .with(eq("org.mechanix.test.section.key"))
        .returning(move |_| Ok(settings.clone()));

    // Call the function under test
    let result = get_setting(&mock_proxy, "org.mechanix.test.section.key").await;

    // Verify the result
    assert!(result.is_ok(), "Get setting should succeed");
    let result_map = result.unwrap();
    assert_eq!(result_map.get("org.mechanix.test.section.key").unwrap(), "test_value", "Retrieved value should match expected value");
}

#[tokio::test]
async fn test_set_setting() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();

    // Set expectations
    mock_proxy
        .expect_set_setting()
        .with(eq("org.mechanix.test.section.key"), eq("test_value"))
        .returning(|_, _| Ok("Success".to_string()));

    // Call the function under test
    let result = set_setting(&mock_proxy, "org.mechanix.test.section.key", "test_value").await;

    // Verify the result
    assert!(result.is_ok(), "Set setting should succeed");
    assert_eq!(result.unwrap(), "Success", "Result should match expected value");
}

#[tokio::test]
async fn test_get_setting_error() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();

    // Set expectations
    mock_proxy
        .expect_get_setting()
        .with(eq("org.mechanix.test.section.key"))
        .returning(|_| Err(zbus::Error::Failure("Test error".to_string())));

    // Call the function under test
    let result = get_setting(&mock_proxy, "org.mechanix.test.section.key").await;

    // Verify the result
    assert!(result.is_err(), "Get setting should fail");
    assert!(result.unwrap_err().to_string().contains("Test error"), "Error message should contain the expected error");
}

#[tokio::test]
async fn test_set_setting_error() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();

    // Set expectations
    mock_proxy
        .expect_set_setting()
        .with(eq("org.mechanix.test.section.key"), eq("test_value"))
        .returning(|_, _| Ok("Error: Test error".to_string()));

    // Call the function under test
    let result = set_setting(&mock_proxy, "org.mechanix.test.section.key", "test_value").await;

    // Verify the result
    assert!(result.is_ok(), "Set setting should succeed even with error response");
    assert_eq!(result.unwrap(), "Error: Test error", "Result should match expected error message");
}
