use std::collections::HashMap;

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

  /// get
  pub fn get(&self, key: String) -> Option<String> {
    self.map.get(&key).cloned()
  }

  /// set
  /// if the key already exist,the previous value will be overwritten
  pub fn set(&mut self, key: String, value: String) -> Option<String> {
    self.map.insert(key, value)
  }

  /// remove
  pub fn remove(&mut self, key: String) {
    self.map.remove(&key).unwrap();
  }
}
