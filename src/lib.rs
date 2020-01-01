#![warn(missing_docs)]

//! This is the rust doc for the `lal` *library* - what the `lal` *binary*
//! depends on to do all the work. This documentation is likely only of use to you
//! if you need to know the internals of `lal` for figuring out how to modify it.
//!
//! ## Testing
//! The library contains all the logic because the binary is only an argument parser,
//! and elaborate decision making engine to log, call one of the libraries functions,
//! then simply `process::exit`.
//! Tests do not cover the binary part, because these failures would be trivially
//! detectable, and also require a subprocess type of testing. Tests instead
//! cover a couple of common use flows through the library.
//!
//!
//! ## Dependencies
//! This tool depends on the rust ecosystem and their crates. Dependencies referenced
//! explicitly or implicitly is listed on the left of this page.

#[macro_use] extern crate hyper;


#[macro_use] extern crate serde_derive;


#[macro_use] extern crate log;


// re-exports
mod core;
pub use crate::core::*;

mod runners;
pub use crate::runners::*;

mod storage;
pub use crate::storage::*;

/// Env module for env subcommand (which has further subcommands)
pub mod env;
/// List module for all the list-* subcommands
pub mod list;
/// Propagation module with all structs describing the steps
pub mod propagate;


// lift most other pub functions into our libraries main scope
// this avoids having to type lal:build in tests and main.rs
pub use crate::{
    build::{build, BuildOptions},
    clean::clean,
    configure::configure,
    export::export,
    fetch::fetch,
    init::init,
    publish::publish,
    query::query,
    remove::remove,
    shell::{run, script, shell},
    stash::stash,
    status::status,
    update::{update, update_all},
    verify::verify,
};

mod build;
mod clean;
mod configure;
mod export;
mod fetch;
mod init;
mod publish;
mod query;
mod remove;
mod shell;
mod stash;
mod status;
mod update;
mod verify;

#[cfg(feature = "upgrade")] pub use upgrade::upgrade;
#[cfg(feature = "upgrade")] mod upgrade;
