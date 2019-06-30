use crate::Result;
use std::collections::HashMap;
use std::path::Path;

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
#[derive(Debug, PartialEq, Default)]
pub struct KvStore {
  map: HashMap<String, String>,
}

impl KvStore {
  /// new
  pub fn new() -> Self {
    KvStore {
      map: HashMap::new(),
    }
  }

  ///
  pub fn open(path: &Path) -> Result<Self> {
    panic!();
  }

  /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
  pub fn get(&self, key: String) -> Result<Option<String>> {
    panic!();
    // self.map.get(&key).cloned()
  }

  /// Set the value of a string key to a string. Return an error if the value is not written successfully.
  /// if the key already exist,the previous value will be overwritten
  pub fn set(&mut self, key: String, value: String) -> Result<()> {
    panic!();
    // self.map.insert(key, value)
  }

  /// Remove a given key. Return an error if the key does not exist or is not removed successfully.
  pub fn remove(&mut self, key: String) -> Result<()> {
    panic!();
    self.map.remove(&key).unwrap();
  }
}
