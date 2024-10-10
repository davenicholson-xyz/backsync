use anyhow::anyhow;
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs, io::Write, path::Path};
use toml::Value;

use super::files;
use super::paths;

pub fn get<T: DeserializeOwned>(field: &str) -> Result<Option<T>> {
    let config_file = paths::config_path("config.toml");
    if Path::new(&config_file).exists() {
        let contents = fs::read_to_string(config_file)?;
        let config: Value = toml::from_str(&contents)?;
        if let Some(value) = config.get(field) {
            if let Ok(deserialized) = value.clone().try_into() {
                return Ok(Some(deserialized));
            }
        }
    }
    Ok(None)
}

pub fn set<T: Serialize>(field: &str, value: T) -> Result<()> {
    let config_file = paths::config_path("config.toml");

    let mut config: Value = if Path::new(&config_file).exists() {
        let contents = fs::read_to_string(&config_file)?;
        toml::from_str(&contents)?
    } else {
        Value::Table(Default::default())
    };

    if let Value::Table(ref mut table) = config {
        let serialized_value = Value::try_from(value).expect("Failed to serialize config value");
        table.insert(field.to_string(), serialized_value);
    } else {
        return Err(anyhow!("Error with TOML table"));
    }

    let updated_content = toml::to_string(&config)?;
    let mut file = fs::File::create(config_file)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

pub fn flag_file_default<T>(flag: Option<T>, field: &str, default: T) -> Result<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    if let Some(flag_value) = flag {
        set(field, flag_value.clone())?;
        return Ok(flag_value);
    }

    if let Some(config_value) = get::<T>(field)? {
        return Ok(config_value);
    }

    set(field, default.clone())?;
    Ok(default)
}

pub fn exists(field: &str) -> Result<bool> {
    let config_file = paths::config_path("config.toml");
    if Path::new(&config_file).exists() {
        let contents = fs::read_to_string(config_file)?;
        let config: Value = toml::from_str(&contents)?;
        if let Some(_value) = config.get(field) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    Ok(false)
}

pub fn set_if_none<T: Serialize>(field: &str, value: T) -> Result<()> {
    if !exists(field)? {
        set(field, value)?
    }
    Ok(())
}
