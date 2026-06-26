use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sid: Option<String>,
}

pub fn load() -> Result<Config> {
    let path = std::env::current_exe()
        .context("実行ファイルのパスが取得できませんでした")?
        .parent()
        .context("実行ファイルの親ディレクトリが取得できませんでした")?
        .join("scrb-config.toml");

    if !path.exists() {
        return Ok(Config { sid: None });
    }

    let text = fs::read_to_string(path)?;

    Ok(toml::from_str(&text)?)
}
