use std::{str::FromStr, time::Duration};

use chatgpt::prelude::{ChatGPT, ModelConfiguration, ChatGPTEngine};
use url::Url;

use crate::base::types::{HasName, IsEnsurable, Mode, Res, Void};

static NAME: &