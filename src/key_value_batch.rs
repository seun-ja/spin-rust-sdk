use crate::wit::wasi::keyvalue::store::KeyResponse;

use super::wit::wasi::keyvalue;

pub use keyvalue::batch::Bucket;

pub use keyvalue::batch::Error;

impl Bucket {
    /// Retrieves a key-value pair from the bucket.
    pub fn batch_get(&self, key: &str) -> Result<Option<Vec<u8>>, anyhow::Error> {
        Ok(self.get(key)?)
    }

    /// Sets a key-value pair in the bucket.
    pub fn batch_set(&self, key: &str, value: &[u8]) -> Result<(), anyhow::Error> {
        Ok(self.set(key, value)?)
    }

    /// Deletes a key-value pair from the bucket.
    pub fn batch_delete(&self, key: &str) -> Result<(), anyhow::Error> {
        Ok(self.delete(key)?)
    }

    /// Check if the key exists in the store.
    ///
    /// If the key exists in the store, it returns Ok(true). If the key does not exist in the store, it returns Ok(false).
    /// If any other error occurs, it returns an Err(error).
    pub fn batch_exists(&self, key: &str) -> Result<bool, anyhow::Error> {
        Ok(self.exists(key)?)
    }

    /// Retrieves all key-value pairs from the bucket.
    pub fn batch_list_keys(&self, cursor: Option<&str>) -> Result<KeyResponse, anyhow::Error> {
        Ok(self.list_keys(cursor)?)
    }

    /// Handles a key-value pair in the bucket.
    pub fn batch_handle(&self) -> Result<u32, anyhow::Error> {
        Ok(self.handle())
    }

    /// Takes a handle to a key-value pair in the bucket.
    pub fn batch_take_handle(&self) -> Result<u32, anyhow::Error> {
        Ok(self.take_handle())
    }
}
