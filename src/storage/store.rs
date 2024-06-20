use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// MemStore is an in-memory Store implementation for cunner framework.
pub struct MemStore {
    lock: RwLock<HashMap<String, Bytes>>, // ensure thread safety and efficient read/write operations
}

impl MemStore {
    /// Returns a new MemoryStore.
    pub fn new_mem_store() -> Arc<Self> {
        Arc::new(Self {
            lock: RwLock::new(HashMap::new()),
        })
    }

    /// Gets a value by key. It returns Option<Bytes> to handle cases where the key might not exist.
    pub fn get_value_from_key(&self, key: &[u8]) -> Option<Bytes> {
        let read_lock = self.lock.read().unwrap();
        read_lock.get(String::from_utf8_lossy(key).as_ref()).cloned()
    }

    /// Puts a key-value pair into the store.
    pub fn put_value_to_key(&self, key: &[u8], val: &[u8]) {
        let mut write_lock = self.lock.write().unwrap();
        write_lock.insert(
            String::from_utf8_lossy(key).to_string(),
            Bytes::copy_from_slice(val),
        );
    }

    /// Checks if a key exists in the store.
    pub fn has_key(&self, key: &[u8]) -> bool {
        let read_lock = self.lock.read().unwrap();
        read_lock.contains_key(String::from_utf8_lossy(key).as_ref())
    }

    /// Returns the number of elements in the store.
    pub fn length(&self) -> usize {
        let read_lock = self.lock.read().unwrap();
        read_lock.len()
    }
}
