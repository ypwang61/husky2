pub mod environment;
pub mod jar;
pub mod menu;
pub mod module;
pub mod namespace;
pub mod path;
#[cfg(test)]
mod tests;

use self::jar::VdEntityPathJar as Jar;
#[cfg(test)]
use crate::tests::*;
