use crate::{Error, Result};
use std::{env, sync::OnceLock};

#[allow(non_snake_case)]
pub struct Config {
    pub DB_URL: String,
    pub WEB_FOLDER: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONFIG: {ex:?}")
        })
    })
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: get_env("SERVICE_DB_URL")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}