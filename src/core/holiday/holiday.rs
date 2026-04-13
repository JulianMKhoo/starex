use chrono::{Datelike, Utc};

use crate::config::config::{get_holiday, get_timezone, get_user_config};

pub fn get_full_holiday_logic() -> String {
    let mut holiday = get_holiday(&get_user_config());
    holiday.sort_by_key(|h| h.date);
    let mut holiday_result = String::new();
    for h in holiday {
        holiday_result += &(h.name + " " + &h.date.to_string() + " " + &h.icon + "\n")
    }

    holiday_result
}

pub fn get_now_holiday_logic() -> String {
    let holiday = get_holiday(&get_user_config());
    let now = Utc::now().with_timezone(&get_timezone(&get_user_config()));
    for h in holiday {
        if h.date.day() == now.day() && h.date.month() == now.month() {
            return String::from(&(h.name + " " + &h.date.to_string() + " " + &h.icon));
        }
    }
    String::from("No Holiday Today")
}

pub fn get_month_holiday_logic(month: u32) -> String {
    let mut holiday = get_holiday(&get_user_config());
    holiday.sort_by_key(|h| h.date);
    let mut holiday_result = String::new();
    let mut month = month;

    if month == 0 {
        month = Utc::now()
            .with_timezone(&get_timezone(&get_user_config()))
            .month();
    }
    for h in holiday {
        if h.date.month() == month {
            holiday_result += &(h.name + " " + &h.date.to_string() + " " + &h.icon + "\n")
        }
    }

    if holiday_result.is_empty() {
        holiday_result = String::from("No Holiday This Month")
    }

    holiday_result
}
