// #![deny(missing_docs)]
//! kv store
#![feature(seek_convenience)]
pub use error::{KvsError, Result};
pub use kv::{KvStore, KvsCommand};

mod error;
mod kv;
