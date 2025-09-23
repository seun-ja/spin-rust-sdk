use crate::wit::wasi::keyvalue::store::KeyResponse;

use super::wit::wasi::keyvalue;

pub use keyvalue::atomics::Bucket;
pub use keyvalue::atomics::Cas;

pub use keyvalue::atomics::CasError;
pub use keyvalue::atomics::Error as AtomicsError;

impl keyvalue::atomics::Bucket {
    /// Opens a new bucket with a given handle.
    pub fn open_bucket(handle: u32) -> Self {
        unsafe { Self::from_handle(handle) }
    }
}

impl keyvalue::atomics::Bucket {
    /// Retrieves a key-value pair from the bucket.
    pub fn atomic_get(&self, key: &str) -> Result<Option<Vec<u8>>, anyhow::Error> {
        Ok(self.get(key)?)
    }

    /// Sets a key-value pair in the bucket.
    pub fn atomic_set(&self, key: &str, value: &[u8]) -> Result<(), anyhow::Error> {
        Ok(self.set(key, value)?)
    }

    /// Deletes a key-value pair from the bucket.
    pub fn atomic_delete(&self, key: &str) -> Result<(), anyhow::Error> {
        Ok(self.delete(key)?)
    }

    /// Checks if a key-value pair exists in the bucket.
    pub fn atomic_exists(&self, key: &str) -> Result<bool, anyhow::Error> {
        Ok(self.exists(key)?)
    }

    /// Retrieves all key-value pairs from the bucket.
    pub fn atomic_list_keys(&self, cursor: Option<&str>) -> Result<KeyResponse, anyhow::Error> {
        Ok(self.list_keys(cursor)?)
    }

    /// Handles a key-value pair in the bucket.
    pub fn atomic_handle(&self) -> Result<u32, anyhow::Error> {
        Ok(self.handle())
    }

    /// Takes a handle to a key-value pair in the bucket.
    pub fn atomic_take_handle(&self) -> Result<u32, anyhow::Error> {
        Ok(self.take_handle())
    }
}

impl keyvalue::atomics::Cas {
    /// Get the current value of the key (if it exists). This allows for avoiding reads if all that is needed to ensure the atomicity of the operation
    pub fn current_value(&self) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(self.current()?)
    }
}
