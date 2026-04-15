use std::{collections::HashMap, env, fs, time::SystemTime};

use chrono::{Datelike, FixedOffset, NaiveDate, Utc};
use once_cell::sync::Lazy;

use crate::{
    config::config_struct::{DefaultConfig, UserConfig},
    core::holiday::holiday_struct::Holiday,
};

const DEFAULT_CONFIG_DATA: &str = include_str!("../../data/starex.toml");

pub static DEFAULT_CONFIG: Lazy<DefaultConfig> = Lazy::new(|| {
    let data: DefaultConfig = toml::from_str(DEFAULT_CONFIG_DATA).expect("CONFIG CONV FAILED");
    data
});

fn get_holiday_user_icon(key: &str, user_icon: &HashMap<String, String>) -> String {
    if let Some(custom_icon) = user_icon.get(key) {
        return custom_icon.clone();
    }

    DEFAULT_CONFIG
        .fixed_holiday
        .get(key)
        .or(DEFAULT_CONFIG.floating_holiday.get(key))
        .map(|h| h.icon.clone())
        .unwrap_or_else(|| "".to_string())
}

pub fn get_user_config_path() -> Option<String> {
    env::var("HOME")
        .ok()
        .map(|h| format!("{}/.config/starex.toml", h))
}

pub fn get_user_config(path: &Option<String>) -> Option<UserConfig> {
    let path = path.as_deref()?;

    let content = fs::read_to_string(path).ok()?;

    toml::from_str(&content).ok()
}

pub fn get_user_config_last_modified(path: &Option<String>) -> Option<SystemTime> {
    fs::metadata(path.as_deref()?).ok()?.modified().ok()
}

pub fn get_timezone(user_config: &Option<UserConfig>) -> FixedOffset {
    let mut hour = DEFAULT_CONFIG.time_offset.timezone.unwrap_or(7);
    let mut direction = DEFAULT_CONFIG.time_offset.direction.clone();

    if let Some(config) = user_config {
        if config.time.direction.is_some() {
            direction = config.time.direction.clone();
        }
        if let Some(tz) = config.time.timezone {
            hour = tz.abs();
        }
    }

    let offset_seconds = hour * 3600;

    if direction.unwrap_or_default() == "west" {
        FixedOffset::west_opt(offset_seconds)
    } else {
        FixedOffset::east_opt(offset_seconds)
    }
    .unwrap_or_else(|| FixedOffset::east_opt(7 * 3600).unwrap())
}

pub fn get_holiday(user_config: &Option<UserConfig>) -> Vec<Holiday> {
    let timezone = get_timezone(user_config);
    let mut holiday_result = Vec::new();
    let current_year = Utc::now().with_timezone(&timezone).year();

    let empty_map = std::collections::HashMap::new();
    let user_icons = user_config
        .as_ref()
        .map(|c| &c.holiday_icon)
        .unwrap_or(&empty_map);

    for (key, holiday_config) in DEFAULT_CONFIG.fixed_holiday.iter() {
        let date_parts: Vec<&str> = holiday_config.date.split('-').collect();
        if date_parts.len() < 2 {
            continue;
        }

        if let (Ok(d), Ok(m)) = (date_parts[0].parse::<u32>(), date_parts[1].parse::<u32>()) {
            if let Some(date) = NaiveDate::from_ymd_opt(current_year, m, d) {
                holiday_result.push(Holiday {
                    name: holiday_config.name.clone(),
                    date,
                    icon: get_holiday_user_icon(key, user_icons),
                });
            }
        }
    }

    for (key, holiday_config) in DEFAULT_CONFIG.floating_holiday.iter() {
        let date_parts: Vec<&str> = holiday_config.date.split('-').collect();
        if date_parts.len() < 3 {
            continue;
        }

        if let (Ok(wd), Ok(wk), Ok(m)) = (
            date_parts[0].parse::<i32>(),
            date_parts[1].parse::<u32>(),
            date_parts[2].parse::<u32>(),
        ) {
            if let Some(date) = get_floating_date(current_year, m, wk, wd) {
                holiday_result.push(Holiday {
                    name: holiday_config.name.clone(),
                    date,
                    icon: get_holiday_user_icon(key, user_icons),
                });
            }
        }
    }

    holiday_result
}

fn get_floating_date(year: i32, month: u32, week: u32, weekday: i32) -> Option<NaiveDate> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_weekday = first_day.weekday();
    let days_to_first_target = (weekday - first_weekday.num_days_from_sunday() as i32 + 7) % 7;
    let target_day = (days_to_first_target as u32) + ((week - 1) * 7) + 1;
    let final_date = NaiveDate::from_ymd_opt(year, month, target_day)?;

    Some(final_date)
}
