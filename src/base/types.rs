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
            "openai" => Ok(Mode::OpenAi),
            _ => Err(anyhow::Error::msg("Invalid mode specified.")),
        }
    }
}

impl Mode {
    pub fn is_openai(&self) -> bool {
        self == &Mode::OpenAi
    }

    pub fn is_local(&self) -> bool {
        self == &Mode::LocalCpu || self == &Mode::LocalGpu
    }

    pub fn is_local_gpu(&self) -> bool {
        self == &Mode::LocalGpu
    }

    pub fn is_local_cpu(&self) -> bool {
        self == &Mode::LocalCpu
    }
}

// Traits for various internal functionality.

pub trait HasName {
    fn name(&self) -> &'static str;
}

pub trait IsEnsurable {
    async fn is_present(&self) -> Result<bool>;
    async fn make_present(&self) -> Result<()>;
}

pub trait IsRemovable {
    async fn make_not_present(&self) -> Result<()>;
}

// "Public" Entity definitions (performs passthrough to helper functions).

pub trait EnsurableEntity {
    async fn ensure(&self, confirm: bool) -> Result<()>;
}

impl<T> EnsurableEntity for T
    where T: HasName + IsEnsurable + Send + Sync
{
    async fn ensure(&self, confirm: bool) -> Result<()> {
        let name = self.name();
        print!("Checking if `{}` is present ... ", Paint::blue(name));

        if self.is_present().await? {
            println!("💯!");
            return Ok(())
        }

        println!("{}!", Paint::red("✘"));
        
        if confirm && !Confirm::new().with_prompt(format!("{}`{}` is not present: do you want me to make it so?", TAB, Paint::blue(name))).interact()? {
            println!("{}Skipping ...", TAB);
            return Err(anyhow::anyhow!("User skipped required operation."));
        }
        
        println!("{}Ensuring presence of `{}` ({}) ...", TAB, Paint::blue(name), Paint::yellow("you may need to interact with the execution"));

        self.make_present().await?;
        
        println!("{}Successfully ensured `{}`.", TAB, Paint::blue(name));
        
        Ok(())
    }
}

pub trait RemovableEntity {
    async fn remove(&self, confirm: bool) -> Result<()>;
}

impl<T> RemovableEntity for T
    where T: HasName + IsEnsurable + IsRemovable + Send + Sync
{
    async fn remove(&self, confirm: bool) -> Result<()> {
        let name = self.name();

        if !self.is_present().await? {
            println!("{} `{}` is not running!", TAB, Paint::blue(name));
            return Ok(())
        }

        if confirm && !Confirm::new().with_prompt(format!("{}`{}` is present: do you want me to remove it?", TAB, Paint::blue(name))).interact()? {
            println!("{}Skipping ...", TAB);
            return Ok(())
        }

        println!("{}Removing presence of `{}` ({}) ...", TAB, Paint::blue(name), Paint::yellow("you may need to interact with the execution [and sudo]"));

        self.make_not_present().await?;
        
        println!("{}Successfully removed `{}`.", TAB, Paint::blue(name));
        
        Ok(())
    }
}

// Exit status helper trait.

pub trait MapStatus {
    fn map_status(self) -> Result<()>;
}

impl MapStatus for Result<ExitStatus, std::io::Error> {
    fn map_status(self) -> Result<()> {


        self
            .map(|s| s.code())
            .map_err(|e| e.into())
            .and_then(|s| match s {
                None | Some(0) => Ok(()),
                Some(c) => Err(anyhow::Error::msg(format!("The exit code of the operation was not successful ({}).", c))),
            })
    }
}

// Other helper methods.

pub(crate) async fn is_binary_present<T>(s: &T) -> Result<bool>
    where T: HasName
{
    let cmd = if cfg!(target_os = "windows") { "where" } else { "which" };

    Ok(Command::new(cmd)
        .arg(s.name())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .status().await?.success())
}