#![deny(missing_docs)]
//! kv store

pub use kv::KvStore;
pub use error::Result;

mod kv;
mod error;
