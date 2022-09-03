use uuid::Uuid;
use std::env;

use crate::twitar::settings::config::{get_configuration, Settings};


fn get_test_config() -> Settings {
    let config = {
        env::set_var("APP_ENV", "test");

        let db_name = Uuid::new_v4().to_string();
        let mut app_config = get_configuration().expect("Failed to read configuration");
        app_config.db.database_name = db_name;

        app_config
    }
    return config;
}