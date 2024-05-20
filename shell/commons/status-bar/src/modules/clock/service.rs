use chrono::Local;

pub struct ClockService {}

impl ClockService {
    pub fn get_current_time(format: &str) -> String {
        //add mctl libs code here
        let time = format!("{}", Local::now().format(format));
        time
    }
}
