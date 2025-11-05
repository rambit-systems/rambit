//! Provides domain model types. Used by most crates in the workspace.

#![feature(never_type)]

mod cache;
mod entry;
mod org;
#[cfg(feature = "session")]
mod session;
mod store;
mod user;

pub use model::{self, RecordId};
pub use model_types::*;
pub use nix_compat;

#[cfg(feature = "session")]
pub use self::session::*;
pub use self::{cache::*, entry::*, org::*, store::*, user::*};
