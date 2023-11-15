use tokio::process::Command;
use anyhow::Context;
use yansi::Paint;

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Res, Void, TAB};

static NAME: &str = "git";

#[derive(Default)]
pub struct Git {}

impl HasName for Git {
    fn nam