#![feature(trait_upcasting)]
#![feature(let_chains)]
mod context;
mod db;
mod error;
mod menu;
mod rewrite;
mod term;
mod utils;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;

use husky_coword::*;
use husky_term_prelude::*;
use husky_vfs::Toolchain;

#[salsa::jar(db =  DeclarativeTermDb)]
pub struct DeclarativeTermJar(
    DeclarativeTermSymbol,
    DeclarativeTermSymbols,
    declarative_term_curry_symbols,
    declarative_term_ritchie_symbols,
    declarative_term_application_symbols,
    DeclarativeTermVariable,
    DeclarativeTermVariables,
    declarative_term_curry_placeholders,
    declarative_term_ritchie_variables,
    declarative_term_application_variables,
    DeclarativeTermCurry,
    curry_parameter_count,
    DeclarativeTermRitchie,
    DeclarativeTermAbstraction,
    DeclarativeTermExplicitApplication,
    DeclarativeTermExplicitApplicationOrRitchieCall,
    DeclarativeTermSubentity,
    DeclarativeTermAsTraitSubentity,
    DeclarativeTermTraitConstraint,
    DeclarativeTermList,
    declarative_term_menu,
    DeclarativeTermWrapper,
);
