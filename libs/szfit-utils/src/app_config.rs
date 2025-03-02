use std::sync::OnceLock;

use std::env;
use std::str::FromStr;

pub fn app_config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AppConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, derive_more::Display, Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

#[allow(non_snake_case)]
pub struct AppConfig {
    // -- Db
    pub DB_URL: String,
    pub TELEGRAM_API_KEY: String,
}

impl AppConfig {
    fn load_from_env() -> Result<AppConfig> {
        println!(
            "Load config from env: {}",
            env::current_dir().unwrap().display()
        );
        Ok(AppConfig {
            // -- Db
            DB_URL: get_env("DATABASE_URL")?,
            // -- Telegram
            TELEGRAM_API_KEY: get_env("TELOXIDE_TOKEN")?,
        })
    }
}

pub fn get_env(name: &'static str) -> Result<String> {
    // получить путь env file
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

#[allow(unused_macros)]
pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}
