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
        Err(anyhow::Error::msg("Cannot perform `make_present`: this should not happen."))
    }
}

impl Gpt {
    pub async fn review(&self, diff: &str) -> Res<String> {
        let key = self.resolve_key()?;

        let url = format!("{}/v1/chat/completions", self.url);
        let config = ModelConfiguration {
            engine: ChatGPTEngine::Gpt4,
            api_url: Url::from_str(&url)?,
            timeout: Duration::from_secs(300),
            ..Default::default()
        };

        let client = ChatGPT::new_with_config(key, config)?;

        let message = REVIEW_PROMPT.replace("{{diff}}", diff);

        let response = client.send_message(message).await?;

        Ok(response.message_choices[0].message.content.clone())
    }

    pub async fn ask(&self, prompt: &str) -> Res<String> {
        let key = self.resolve_key()?;

        let url = format!("{}/v1/chat/completions", self.url);
        let config = ModelConfiguration {
            engine: ChatGPTEngine::Gpt4,
            api_url: Url::from_str(&url)?,
            timeout: