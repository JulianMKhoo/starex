use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::cache_struct::AppState,
    config::config::{get_user_config, get_user_config_last_modified, get_user_config_path},
};

pub fn init_cache() {
    let user_config_path = get_user_config_path();

    let app_cache = Arc::new(Mutex::new(AppState {
        user_config: get_user_config(&user_config_path),
        data_cache: HashMap::new(),
        toggle_cache: HashMap::new(),
        last_config_modified: get_user_config_last_modified(&user_config_path),
    }));
}
