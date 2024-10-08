use crate::system;
use anyhow::anyhow;
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::{fs, io::Write, path::Path};
use toml::Value;

pub fn get<T: DeserializeOwned>(field: &str) -> Result<Option<T>> {
    create_if_none()?;
    let filepath = system::files::config_file()?;
    if Path::new(&filepath).exists() {
        let contents = fs::read_to_string(filepath)?;
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
    create_if_none()?;

    let filepath = system::files::config_file()?;

    let mut config: Value = if Path::new(&filepath).exists() {
        let contents = fs::read_to_string(&filepath)?;
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
    let mut file = fs::File::create(filepath)?;
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

#[allow(dead_code)]
pub fn exists(field: &str) -> Result<bool> {
    let filepath = system::files::config_file()?;
    if Path::new(&filepath).exists() {
        let contents = fs::read_to_string(filepath)?;
        let config: Value = toml::from_str(&contents)?;
        if let Some(_value) = config.get(field) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    Ok(false)
}

#[allow(dead_code)]
pub fn set_if_none<T: Serialize>(field: &str, value: T) -> Result<()> {
    if !exists(field)? {
        set(field, value)?
    }
    Ok(())
}

fn create_if_none() -> Result<()> {
    let config_path = system::files::config_path()?;
    let config_file = system::files::config_file()?;

    fs::create_dir_all(config_path)?;
    if !Path::new(&config_file).exists() {
        File::create(&config_file)?;
    }
    Ok(())
}
