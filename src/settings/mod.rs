use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

use crate::generic;

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub radius: i32,
    pub file: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub dburl: String,
    pub dbport: u16,
    pub dbname: String,
    pub dborg: String,
    pub dbapi: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Color {
    pub green: String,
    pub yellow: String,
    pub orange: String,
    pub red: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub env: ENV,
    pub db: Db,
    pub location: Location,
    pub color: Color,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(&format!("{}/config/Default.toml", generic::get_current_working_dir())))?;
        s.merge(File::with_name(&format!("{}/config/{}", generic::get_current_working_dir(), env)))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}
