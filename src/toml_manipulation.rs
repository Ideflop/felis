use toml::{map::Map, Value};

use crate::config::check_config;

use std::{
    fs,
    str::FromStr,
};

pub struct Toml;

impl Toml {
    pub fn read_config() -> Result<Value, String> {
        let config_path = check_config(); // in config.rs
        let config_read = match fs::read_to_string(config_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("could not read config file: {}", e)),
        };
        match Value::from_str(&config_read) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("could not parse config file: {}", e)),
        }
    }

    pub fn get_value(key: &str) -> Result<String, String> {
        let config = match Toml::read_config() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let value = match config.get(key) {
            Some(v) => v,
            None => return Err(format!("key not found in config file: {}", key)),
        };
        match value.as_str() {
            Some(s) => Ok(s.to_owned()),
            None => Err(format!("value for key {} is not a string", key)),
        }
    }

    pub fn create_toml_table<K, V>(key: K, value: V) -> Value
        where
            K: Into<String>,
            V: Into<Value>,
        {
        let mut table = Map::new();
        table.insert(key.into(), value.into());
        Value::Table(table)
    }

    pub fn update_value(key: &str, value: &str) -> Result<(), String> {
        let mut config = match Toml::read_config() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        match config {
            Value::Table(mut table) => {
                table.insert(key.to_owned(), Value::String(value.to_owned()));
                config = Value::Table(table);
            }
            _ => return Err("config is not a table".to_owned()),
        }
        let config_string = config.to_string();
        match fs::write(check_config(), config_string) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("could not write to config file: {}", e)),
        }
    }
}
