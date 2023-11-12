
// Main entrypoint.

// Directives.
#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(fs_try_exists)]
#![feature(vec_into_raw_parts)]

// Modules.

pub mod base;
pub mod services;

// Imports.

use base::{types::{Void, EnsurableEntity, Mode, RemovableEntity}, config::Config};
use clap::{command, Parser, Subcommand};
use services::{git::Git, gpt::Gpt, cria::Cria};
use termimad::MadSkin;
use yansi::Paint;

use crate::services::{docker::Docker, model::Model};

// Commands.

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The path to the data directory.
    #[arg(short, long, default_value = ".augre")]
    data_path: String,

    /// The default operation mode.
    #[arg(short, long, default_value = "openai")]
    mode: Mode,

    /// Whether to skip the confirmation prompt.
    #[clap(long = "yes", short = 'y', action)]
    skip_confirm: bool,

    #[command(subcommand)]
    command: Option<Command>,