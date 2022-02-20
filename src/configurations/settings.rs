use serde::{Deserialize};

use super::{db_settings::DatabaseSettings, application_settings::ApplicationSettings};



#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}