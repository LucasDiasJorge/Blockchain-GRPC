use rocksdb::{IteratorMode, Options, DB};
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

/// Low-level RocksDB adapter (Adapter Pattern)
/// Encapsulates RocksDB operations
pub struct RocksDbAdapter {
    db: Arc<DB>,
}

impl RocksDbAdapter {
    /// Creates a new RocksDB adapter
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open(&opts, path)?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Puts a key-value pair
    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }

    /// Gets a value by key
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        match self.db.get(key.as_bytes())? {
            Some(value) => Ok(Some(value.to_vec())),
            None => Ok(None),
        }
    }

    /// Deletes a key
    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    /// Checks if a key exists
    pub fn exists(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        Ok(self.db.get(key.as_bytes())?.is_some())
    }

    /// Gets all keys with a given prefix
    pub fn get_keys_with_prefix(&self, prefix: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let mut keys = Vec::new();
        let iter = self.db.iterator(IteratorMode::Start);

        for item in iter {
            let (key, _) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with(prefix) {
                keys.push(key_str);
            }
        }

        Ok(keys)
    }

    /// Gets all values with a given key prefix
    pub fn get_values_with_prefix(&self, prefix: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let mut values = Vec::new();
        let iter = self.db.iterator(IteratorMode::Start);

        for item in iter {
            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with(prefix) {
                values.push(value.to_vec());
            }
        }

        Ok(values)
    }

    /// Performs a batch write operation
    pub fn batch_put(&self, items: Vec<(String, Vec<u8>)>) -> Result<(), Box<dyn Error>> {
        let mut batch = rocksdb::WriteBatch::default();

        for (key, value) in items {
            batch.put(key.as_bytes(), &value);
        }

        self.db.write(batch)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_rocksdb_operations() {
        let dir = tempdir().unwrap();
        let adapter = RocksDbAdapter::new(dir.path()).unwrap();

        // Test put and get
        adapter.put("test_key", b"test_value").unwrap();
        let value = adapter.get("test_key").unwrap();
        assert_eq!(value, Some(b"test_value".to_vec()));

        // Test exists
        assert!(adapter.exists("test_key").unwrap());
        assert!(!adapter.exists("non_existent").unwrap());

        // Test delete
        adapter.delete("test_key").unwrap();
        assert!(!adapter.exists("test_key").unwrap());
    }

    #[test]
    fn test_prefix_operations() {
        let dir = tempdir().unwrap();
        let adapter = RocksDbAdapter::new(dir.path()).unwrap();

        adapter.put("prefix_key1", b"value1").unwrap();
        adapter.put("prefix_key2", b"value2").unwrap();
        adapter.put("other_key", b"value3").unwrap();

        let keys = adapter.get_keys_with_prefix("prefix_").unwrap();
        assert_eq!(keys.len(), 2);
    }
}
