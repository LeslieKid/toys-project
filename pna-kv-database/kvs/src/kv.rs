use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf, process::exit};

use serde::{Deserialize, Serialize};
use crate::err::{KvsError, Result};

#[derive(Serialize, Deserialize)]
enum KvOps {
    Set(String, String),
    Get(String),
    Rm(String),
}

pub struct KvStore {
    kv_map: HashMap<String, String>,
    storage_pos: PathBuf,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            kv_map: HashMap::new(),
            storage_pos: PathBuf::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = KvOps::Set(key, value);
        let serialized = serde_json::to_string(&set_cmd)?;
        let mut file = OpenOptions::new().append(true).open(&self.storage_pos)?;
        file.write_all(serialized.as_bytes()).expect("Write to the target file unsuccessfully.");

        exit(0);
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        unimplemented!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let result = self.kv_map.remove(&key).ok_or("Key not found");
        match result {
            Ok(_string) => { Ok(()) }
            Err(_err) => { Err(KvsError::NotFound) }
        } 
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore {
            kv_map: HashMap::new(),
            storage_pos: path.into(),
        })
    }
}
