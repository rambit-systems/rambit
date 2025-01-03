//! Provides access to the database.
//!
//! This is a hexagonal crate. It provides a [`DatabaseAdapter`] trait, and
//! implementers.
//!
//! # [`DatabaseAdapter`]
//! The [`DatabaseAdapter`] trait provides a **tabular** database interface. It
//! provides CRUD operations for a generic item, using the
//! [`Model`](models::Model) trait to carry the table information.
//!
//! The implementation of the [`DatabaseAdapter`] trait is responsible for
//! organizing the database, and for bridging the gap between raw data in the kv
//! store and the model data.
//!
//! Admittedly, this is a little bit of a leaky abstraction. It
//! makes use of the [`Model`](models::Model) trait, which ideally would not be
//! involved at this level, since it's involved in the domain and the
//! [`db`](crate) crate is not.
//!
//! # Errors
//! Ideally, each method in [`DatabaseAdapter`] should return a specific,
//! concrete error. This is the case for all but
//! [`enumerate_models`](DatabaseAdapter::enumerate_models), which returns a
//! [`miette::Report`].
//!
//! # Implementers
//! The [`KvDatabaseAdapter`] is the only implementer of the [`DatabaseAdapter`]
//! trait. It's generic on a [`KvTransactional`](kv::prelude::KvTransactional)
//! implementation.

mod adapter;
mod kv_impl;
#[cfg(feature = "migrate")]
mod migrate;

pub use kv;

#[cfg(feature = "migrate")]
pub use self::migrate::Migratable;
pub use self::{adapter::*, kv_impl::KvDatabaseAdapter};
