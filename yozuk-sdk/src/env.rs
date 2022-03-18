use anyhow::{anyhow, Result};
use jsonschema::JSONSchema;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use slog::Logger;
use sloggers::null::NullLoggerBuilder;
use sloggers::Build;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub build_info: &'static str,
    pub logger: Logger,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            build_info: "",
            logger: NullLoggerBuilder.build().unwrap(),
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_info(mut self, build_info: &'static str) -> Self {
        self.build_info = build_info;
        self
    }

    pub fn logger(mut self, logger: Logger) -> Self {
        self.logger = logger;
        self
    }
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
    pub fn new(value: &Value, schema: &str) -> Result<Self> {
        let schema: Value = serde_json::from_str(schema)?;
        let schema = JSONSchema::compile(&schema).map_err(|err| anyhow!("{}", err))?;
        schema
            .validate(value)
            .map_err(|err| anyhow!("{:?}", err.collect::<Vec<_>>()))?;
        Ok(Self {
            data: serde_json::to_string(value)?,
        })
    }

    pub fn get<'a, T>(&'a self) -> T
    where
        T: serde::de::Deserialize<'a> + Default,
    {
        serde_json::from_str(&self.data).unwrap_or_default()
    }
}
