use tokio::process::Command;
use anyhow::Context;
use yansi::Paint;

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Res, Void, TAB};
