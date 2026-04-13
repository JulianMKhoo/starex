use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct HolidayConfig {
    pub name: String,
    pub date: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TimeCycleConfig {
    pub icon: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TimeOffsetConfig {
    pub timezone: Option<i32>,
    pub direction: Option<String>,
    pub format: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct DefaultConfig {
    pub time_offset: TimeOffsetConfig,
    pub time_cycle: HashMap<String, TimeCycleConfig>,
    pub fixed_holiday: HashMap<String, HolidayConfig>,
    pub floating_holiday: HashMap<String, HolidayConfig>,
}

#[derive(Deserialize, Default)]
pub struct UserConfig {
    #[serde(rename = "time_offset", default)]
    pub time: TimeOffsetConfig,
    #[serde(default)]
    pub holiday_icon: HashMap<String, String>,
    #[serde(default)]
    pub time_icon: HashMap<String, String>,
}
