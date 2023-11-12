
use std::{path::Path, borrow::Cow, time::Duration};

use tokio::process::Command;
use anyhow::Context;

use crate::base::types::{HasName, IsEnsurable, MapStatus, Void, Res, Mode, IsRemovable, TAB};

static NAME: &str = "cria_server";

pub struct Cria {
    model_path: Option<String>,
    data_path: String,
    mode: Mode,
    port: Option<u16>
}

impl HasName for Cria {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Cria {
    async fn is_present(&self) -> Res<bool> {
        let port = self.resolve_port()?;

        // grep for `cria` in `docker ps` output.
        let output = Command::new("curl")
            .arg(format!("http://localhost:{}/v1/models", port))
            .output().await
            .context("Unable to curl the local server.")?;

        Ok(output.status.success())
    }

    async fn make_present(&self) -> Void {
        let _ = self.resolve_port()?;
        let model_path = self.resolve_model_path()?;
        let model_path = Path::new(model_path);

        let path = if cfg!(target_os = "windows") && model_path.is_relative() {
            let interim = model_path.canonicalize()?.to_owned().to_string_lossy().to_string();
            let drive = interim.chars().collect::<Vec<_>>()[4].to_ascii_lowercase();
            let path = format!("//{}{}", drive, &interim[6..].replace('\\', "/"));
            
            Cow::Owned(path)
        } else {
            model_path.to_string_lossy()
        };

        let compose_template = if self.mode.is_local_gpu() {
            GPU_COMPOSE
        } else {
            CPU_COMPOSE
        };

        let compose = compose_template
            .replace("{{port}}", &self.resolve_port()?.to_string())