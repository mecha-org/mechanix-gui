use anyhow::Result;

pub struct WindowService {}

impl WindowService {
    pub async fn get_current_window() -> Result<String> {
        //add gtk window code here

        Ok("Settings".to_string())
    }
}
