use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::process::Command;
use std::{process::{Stdio, ExitStatus}, str::FromStr};
use yansi::Paint;
use dialoguer::Confirm;

// Statics.

pub static TAB: &str = "  ";

// Error helpers.

pub type Res<T> = Result<T, anyhow::Error>;
pub type Void = Res<()>;

// Mode helpers.

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    LocalCpu,
    LocalGpu,
    #[default]
    OpenAi
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "localcpu" => Ok(Mode::LocalCpu),
            "localgpu" => Ok(Mode::LocalGpu),
            "openai" => Ok(Mode::Op