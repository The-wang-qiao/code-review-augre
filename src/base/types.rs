use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::process::Command;
use std::{process::{Stdio, ExitStatus}, str::FromStr};
use yansi::Paint;
use dialoguer::Confirm;

// Statics.

pub static TAB: &str = "  ";

// Error helpers.

pub type Res<T> = Result<T, anyhow::