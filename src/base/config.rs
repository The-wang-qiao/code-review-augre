//! The configuration module.

use config::{Environment, File};
use serde::{Deserialize, Serialize};

use super::types::{Res, Mode};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OptionalConfig {
    openai_