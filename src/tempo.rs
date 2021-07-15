use chrono::{DateTime, Local};
pub fn tempo() -> u32 {
    let data: DateTime<Local>= Local::now();
    let datatime_hour = data.format("%H").to_string();
    let time_now:u32 = datatime_hour.trim().parse().expect("msg");

    return time_now;

}

