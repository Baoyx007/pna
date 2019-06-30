// #![deny(missing_docs)]
//! kv store

pub use error::{KvsError, Result};
pub use kv::{KvStore,KvsCommand };

mod error;
mod kv;

