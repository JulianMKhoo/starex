use chrono::{Timelike, Utc};

use crate::config::config::{get_timezone, get_user_config, get_user_config_path};

pub fn get_time_logic() -> String {
    let path = &get_user_config_path();
    let utc_time = Utc::now().with_timezone(&get_timezone(&get_user_config(&path)));
    let hour = utc_time.hour();
    let icon_str: &str;
    let gap = " ";

    if hour >= 22 || hour < 5 {
        icon_str = "🌙";
    } else if hour >= 7 && hour < 16 {
        icon_str = "🏙️";
    } else if hour >= 16 && hour < 18 {
        icon_str = "🌇";
    } else if hour >= 18 && hour <= 22 {
        icon_str = "🌃";
    } else {
        icon_str = "🌆";
    }

    let time_str = utc_time.format("%H:%M:%S").to_string();
    String::from(icon_str) + gap + &time_str
}
