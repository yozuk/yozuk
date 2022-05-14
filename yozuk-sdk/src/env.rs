use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    pub build_info: HashMap<String, BuildInfo>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_info<T>(mut self, name: T, build_info: BuildInfo) -> Self
    where
        T: Into<String>,
    {
        self.build_info.insert(name.into(), build_info);
        self
    }

    pub fn merge(self, other: Self) -> Self {
        let mut build_info = self.build_info;
        build_info.extend(other.build_info);
        Self { build_info }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BuildInfo {
    pub version: String,
    pub commit: String,
    pub rustc: String,
    pub timestamp: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub skills: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct SkillConfig {
    data: String,
}

impl Default for SkillConfig {
    fn default() -> Self {
        Self { data: "{}".into() }
    }
}

impl SkillConfig {
    pub fn new(data: &Value) -> Result<Self> {
        Ok(Self {
            data: serde_json::to_string(data)?,
        })
    }

    pub fn get<'a, T>(&'a self) -> T
    where
        T: serde::de::Deserialize<'a> + Default,
    {
        serde_json::from_str(&self.data).unwrap_or_default()
    }
}
