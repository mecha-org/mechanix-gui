use anyhow::Result;

pub struct InputService {}

impl InputService {
    pub async fn get_current_window() -> Result<String> {
        //add gtk window code here

        Ok("Settings".to_string())
    }
}
