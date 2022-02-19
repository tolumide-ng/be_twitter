use serde::{Deserialize};

use super::db_settings::DatabaseSettings;



#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    
}