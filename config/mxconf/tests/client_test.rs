use mechanix_conf_server::client::{get_setting, set_setting};
use mockall::predicate::*;
use mockall::mock;
use std::sync::Arc;

// Mock the D-Bus proxy
mock! {
    pub ConfigServerProxy {
        pub async fn get_setting(&self, key: &str) -> Result<String, zbus::Error>;
        pub async fn set_setting(&self, key: &str, value: &str) -> String;
    }
}

// We need to manually implement Clone for the mock
impl Clone for MockConfigServerProxy {
    fn clone(&self) -> Self {
        MockConfigServerProxy::new()
    }
}

// Create a module to mock the zbus proxy creation
#[cfg(test)]
mod zbus_mock {
    use super::*;
    use std::sync::Mutex;
    
    // Global mock proxy that will be returned by new_proxy
    static MOCK_PROXY: Mutex<Option<MockConfigServerProxy>> = Mutex::new(None);
    
    // Function to set the mock proxy
    pub fn set_mock_proxy(proxy: MockConfigServerProxy) {
        let mut guard = MOCK_PROXY.lock().unwrap();
        *guard = Some(proxy);
    }
    
    // Function to get the mock proxy
    pub fn get_mock_proxy() -> MockConfigServerProxy {
        let guard = MOCK_PROXY.lock().unwrap();
        guard.as_ref().expect("Mock proxy not set").clone()
    }
}

// Tests for the client module
#[tokio::test]
async fn test_get_setting() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();
    
    // Set expectations
    mock_proxy
        .expect_get_setting()
        .with(eq("org.mechanix.test.section.key"))
        .returning(|_| Ok("test_value".to_string()));
    
    // Set the mock proxy
    zbus_mock::set_mock_proxy(mock_proxy);
    
    // Call the function under test
    let result = get_setting("org.mechanix.test.section.key").await;
    
    // Verify the result
    assert!(result.is_ok(), "Get setting should succeed");
    assert_eq!(result.unwrap(), "test_value", "Retrieved value should match expected value");
}

#[tokio::test]
async fn test_set_setting() {
    // Create a mock proxy
    let mut mock_proxy = MockConfigServerProxy::new();
    
    // Set expectations
    mock_proxy
        .expect_set_setting()
        .with(eq("org.mechanix.test.section.key"), eq("test_value"))
        .returning(|_, _| "Success".to_string());
    
    // Set the mock proxy
    zbus_mock::set_mock_proxy(mock_proxy);
    
    // Call the function under test
    let result = set_setting("org.mechanix.test.section.key", "test_value").await;
    
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
    
    // Set the mock proxy
    zbus_mock::set_mock_proxy(mock_proxy);
    
    // Call the function under test
    let result = get_setting("org.mechanix.test.section.key").await;
    
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
        .returning(|_, _| "Error: Test error".to_string());
    
    // Set the mock proxy
    zbus_mock::set_mock_proxy(mock_proxy);
    
    // Call the function under test
    let result = set_setting("org.mechanix.test.section.key", "test_value").await;
    
    // Verify the result
    assert!(result.is_ok(), "Set setting should succeed even with error response");
    assert_eq!(result.unwrap(), "Error: Test error", "Result should match expected error message");
}