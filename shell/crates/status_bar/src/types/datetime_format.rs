use chrono::{Datelike, Timelike};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateTimeFormat {
    DayMonthTime,     // "20 November 14:30"
    MonthDayTime,     // "November 20 14:30"
    ShortDateTime,    // "20 Nov 14:30"
    ShortDateTimeSec, // "20 Nov 14:30:00"
    Time24Only,       // "14:30"
    Time12Only,       // "2:30"
    Time12Hour,       // "2:30 PM"
    Time12HourSec,    // "2:30:00 PM"
    FullDateTime,     // "20 November 2025 14:30"
}

impl DateTimeFormat {
    pub fn format_datetime(&self, now: &chrono::DateTime<chrono::Local>) -> String {
        match self {
            DateTimeFormat::DayMonthTime => format!(
                "{} {} {:02}:{:02}",
                now.day(),
                now.format("%B"),
                now.hour(),
                now.minute(),
            ),
            DateTimeFormat::MonthDayTime => format!(
                "{} {} {:02}:{:02}",
                now.format("%B"),
                now.day(),
                now.hour(),
                now.minute(),
            ),
            DateTimeFormat::ShortDateTime => format!(
                "{} {} {:02}:{:02}",
                now.day(),
                now.format("%b"),
                now.hour(),
                now.minute(),
            ),
            DateTimeFormat::ShortDateTimeSec => format!(
                "{} {} {:02}:{:02}:{:02}",
                now.day(),
                now.format("%b"),
                now.hour(),
                now.minute(),
                now.second(),
            ),
            DateTimeFormat::Time24Only => format!("{:02}:{:02}", now.hour(), now.minute(),),
            DateTimeFormat::Time12Only => {
                let hour = now.hour();
                let (hour_12, _) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };
                format!("{:02}:{:02} ", hour_12, now.minute())
            },
            DateTimeFormat::Time12Hour => {
                let hour = now.hour();
                let (hour_12, period) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };
                format!("{:02}:{:02} {}", hour_12, now.minute(), period)
            }
            DateTimeFormat::Time12HourSec => {
                let hour = now.hour();
                let (hour_12, period) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };
                format!(
                    "{:02}:{:02}:{:02} {}",
                    hour_12,
                    now.minute(),
                    now.second(),
                    period
                )
            }
            DateTimeFormat::FullDateTime => format!(
                "{} {} {} {:02}:{:02}",
                now.day(),
                now.format("%B"),
                now.year(),
                now.hour(),
                now.minute(),
            ),
        }
    }
}
