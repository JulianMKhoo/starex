use std::{borrow::Cow, collections::HashMap, time::SystemTime};

use crate::{
    cache::cache_enum::{CacheDataKey, CacheToggleKey},
    config::config_struct::UserConfig,
};

pub struct AppState {
    pub user_config: Option<UserConfig>,
    pub data_cache: HashMap<CacheDataKey, Cow<'static, String>>,
    pub toggle_cache: HashMap<CacheToggleKey, bool>,
    pub last_config_modified: Option<SystemTime>,
}
