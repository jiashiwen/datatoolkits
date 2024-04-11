use anyhow::{Ok, Result};
use serde::Deserialize;
use serde_yml::from_str;
use std::fs;

pub fn struct_to_yml_file<T>(value: &T, path: &str) -> Result<()>
where
    T: ?Sized + serde::Serialize,
{
    let yml = serde_yml::to_string(value)?;
    fs::write(path, yml)?;
    Ok(())
}

pub fn struct_to_yaml_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let yml = serde_yml::to_string(value)?;
    Ok(yml)
}

pub fn read_yaml_file<T>(path: &str) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let contents = fs::read_to_string(path)?;
    let r = from_str::<T>(contents.as_str())?;
    Ok(r)
}
