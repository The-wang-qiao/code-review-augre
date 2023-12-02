use tokio::process::Command;
use anyhow::Context;
use yansi::Paint;

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Res, Void, TAB};

static NAME: &str = "git";

#[derive(Default)]
pub struct Git {}

impl HasName for Git {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Git {
    async fn is_present(&self) -> Res<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Void {
        if cfg!(target_os = "windows") {
       