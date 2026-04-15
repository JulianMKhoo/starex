#[derive(Hash, PartialEq, Eq, Debug)]
pub enum CacheDataKey {
    Time,
    TimeToggle,
    Holiday,
    HolidayToggle,
    Weather,
    WeatherToggle,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum CacheToggleKey {
    Time,
    TimeToggle,
    Holiday,
    HolidayToggle,
    Weather,
    WeatherToggle,
}
