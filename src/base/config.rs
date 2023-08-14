//! The configuration module.

use config::{Environment, File};
use serde::{Deserialize, Serialize};

use super::types::{Res, Mode};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OptionalConfig {
    openai_key: Option<String>,
    mode: Option<Mode>,
    model_url: Option<String>,
    cria_port: Option<u16>,
}

/// The configuration type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub openai_endpoint: String,
    pub mode: Mode,
    pub data_path: String,

    pub cria_port: Option<u16>,
    pub openai_key: Option<String>,
    pub model_url: Option<String>,
    pub model_path: Option<String>,
}

impl 