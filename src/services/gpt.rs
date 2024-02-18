use std::{str::FromStr, time::Duration};

use chatgpt::prelude::{ChatGPT, ModelConfiguration, ChatGPTEngine};
use url::Url;

use crate::base::types::{HasName, IsEnsurable, Mode, Res, Void};

static NAME: &str = "gpt_sdk";

pub struct Gpt {
    url: String,
    key: Option<String>,
    mode: Mode,
}

impl HasName for Gpt {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Gpt {
    async fn is_present(&self) -> Res<bool> {
        let _ = self.resolve_key()?;
        
        Ok(true)
    }

    async fn make_present(&self) -> Void {
        Err(anyhow::Error::msg("Cannot perform `make_present`: this shoul