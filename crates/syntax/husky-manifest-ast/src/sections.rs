mod dependencies;
mod dev_dependencies;
mod package;

pub(crate) use self::dependencies::*;
pub(crate) use self::dev_dependencies::*;
pub(crate) use self::package::*;

use husky_coword::Coword;
use husky_toml_token::TomlTokenIdx;
use vec_like::VecPairMap;

use crate::*;
