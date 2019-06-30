use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;

/// in memory KvStore
/// ```rust
/// use kvs::KvStore;
/// let mut store = KvStore::new();
///
/// store.set("key".to_owned(), "value".to_owned());
/// assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
///
/// store.remove("key".to_owned());
/// assert_eq!(store.get("key".to_owned()), None);
/// ```
#[derive(Debug)]
pub struct KvStore {
  map: HashMap<String, String>,
  wirter: BufWriter<File>,
}

impl KvStore {
  /// new
  // pub fn new() -> Self {
  //   KvStore {
  //     map: HashMap::new(),
  //     wirter: None,
  //   }
  // }

  ///
  pub fn open(path: &Path) -> Result<Self> {
    let f = OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open(path)?;
    let wirter = BufWriter::new(f);
    Ok(KvStore {
      map: HashMap::new(),
      wirter,
    })
  }

  /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
  pub fn get(&self, key: String) -> Result<Option<String>> {
    // serde_json::ser::to_string(KvsCommand::Get { key})
    panic!();
    // self.map.get(&key).cloned()
  }

  /// Set the value of a string key to a string. Return an error if the value is not written successfully.
  /// if the key already exist,the previous value will be overwritten
  pub fn set(&mut self, key: String, value: String) -> Result<()> {
    let ser = serde_json::ser::to_string(&KvsCommand::Set { key, value })?;
    self.wirter.write_all(ser.as_bytes())?;
    Ok(())
  }

  /// Remove a given key. Return an error if the key does not exist or is not removed successfully.
  pub fn remove(&mut self, key: String) -> Result<()> {
    panic!();
    self.map.remove(&key).unwrap();
  }
}

#[derive(StructOpt, Debug, Deserialize, Serialize)]
#[structopt(rename_all = "kebab_case")]
pub enum KvsCommand {
  Set { key: String, value: String },
  Get { key: String },
  Rm { key: String },
}
