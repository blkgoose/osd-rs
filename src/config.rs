use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(rename = "watcher")]
    pub watchers: Vec<Watcher>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "source")]
#[serde(rename_all = "snake_case")]
pub enum Watcher {
    File(FileConfig),
    Poll(PollConfig),
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone)]
pub struct FileConfig {
    pub path: String,
    #[serde(default = "interval_default")]
    pub interval: u64,
    pub kind: Kind,
    #[serde(default = "max_default")]
    pub max: i32,
    #[serde(default)]
    pub min: i32,
    #[serde(default)]
    pub display_with: DisplayMethod,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone)]
pub struct PollConfig {
    pub command: String,
    #[serde(default = "interval_default")]
    pub interval: u64,
    pub kind: Kind,
    #[serde(default = "max_default")]
    pub max: i32,
    #[serde(default)]
    pub min: i32,
    #[serde(default)]
    pub display_with: DisplayMethod,
}

fn interval_default() -> u64 {
    100
}

fn max_default() -> i32 {
    100
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Brightness,
    Volume,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub enum DisplayMethod {
    #[serde(rename = "notify-send")]
    #[default]
    NotifySend,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
