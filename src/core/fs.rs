use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let data = serde_json::to_string_pretty(value)?;
    fs::write(path, data)?;
    Ok(())
}
