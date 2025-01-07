use std::fs::{self, File};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use crate::{password, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub server: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
}

pub trait Config: Serialize + DeserializeOwned {
    fn load(path: &str) -> Result<Self> {
        Ok(serde_json::from_reader(File::open(path)?)?)
    }

    fn load_or_default(path: Option<&str>) -> Result<Self>
    where
        Self: Default,
    {
        match path {
            Some(path) => Self::load(path),
            None => Ok(Self::default()),
        }
    }

    fn save(&self, path: &str) -> Result<()> {
        Ok(fs::write(path, &serde_json::to_string_pretty(self)?)?)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Some("0.0.0.0".to_string()),
            port: Some(59999),
            password: Some(password::new()),
        }
    }
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            host: Some("127.0.0.1".to_string()),
            port: Some(9998),
            server: None,
            password: None,
        }
    }
}

impl Config for ServerConfig {}

impl Config for LocalConfig {}
