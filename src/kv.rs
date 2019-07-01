use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;
use std::io::{BufReader, BufWriter};
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
  wirter: BufWriter<File>,
  reader: BufReader<File>,
  log_pointer: HashMap<String, u64>,
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
    let mut reader = BufReader::new(File::open(path)?);
    let log_pointer = build_pointer(&mut reader);

    let wirter = BufWriter::new(f);
    Ok(KvStore {
      log_pointer,
      wirter,
      reader,
    })
  }

  /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
  pub fn get(&mut self, key: String) -> Result<Option<String>> {
    if let Some(&index) = self.log_pointer.get(&key) {
      self.reader.seek(SeekFrom::Start(index))?;
      println!("{:?}", self.reader.stream_position());
      let cmd = serde_json::Deserializer::from_reader(&mut self.reader)
        .into_iter::<KvsCommand>()
        .next()
        .unwrap()?;
      println!("{:?}", cmd);

      if let KvsCommand::Set { value, .. } = cmd {
        Ok(Some(value))
      } else {
        Err(KvsError::NotFound(key))
      }
    } else {
      // Err(KvsError::NotFound(key))
      Ok(None)
    }
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
    // self.map.remove(&key).unwrap();
  }
}

fn build_pointer(reader: &mut BufReader<File>) -> HashMap<String, u64> {
  let mut ret = HashMap::new();
  // serde_json::Deserializer::from_reader(reader).in
  let mut stream = serde_json::de::Deserializer::from_reader(reader).into_iter::<KvsCommand>();

  let mut offset = 0u64;
  while let Some(Ok(cmd)) = stream.next() {
    match cmd {
      KvsCommand::Set { key, .. } => {
        ret.insert(key, offset);
      }
      // KvsCommand::Get { key } => {
      //   ret.get(&key);
      // }
      KvsCommand::Rm { key } => {
        ret.remove(&key);
      }
      _ => {}
    }
    offset = stream.byte_offset() as u64
  }

  println!("{:?}", ret);
  ret
}

#[derive(StructOpt, Debug, Deserialize, Serialize)]
#[structopt(rename_all = "kebab_case")]
pub enum KvsCommand {
  Set { key: String, value: String },
  Get { key: String },
  Rm { key: String },
}
