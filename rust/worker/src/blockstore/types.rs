use core::panic;
use std::hash::{Hash, Hasher};

use crate::errors::ChromaError;

#[derive(Clone)]
pub(crate) struct BlockfileKey<K: PartialEq + PartialOrd + Clone> {
    prefix: String,
    key: K,
}

impl<K: Hash + PartialOrd + Clone> Hash for BlockfileKey<K> {
    // Hash is only used for the HashMap implementation, which is a test/reference implementation
    // Therefore this hash implementation is not used in production and allowed to be
    // hacky
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prefix.hash(state);
    }
}

impl<K: PartialOrd + Clone> PartialEq for BlockfileKey<K> {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix && self.key == other.key
    }
}

impl<K: PartialOrd + Clone> Eq for BlockfileKey<K> {}

pub(crate) trait BlockfileValue {}

impl BlockfileValue for String {}

pub(crate) trait Blockfile<K: PartialEq + PartialOrd + Clone, V: BlockfileValue> {
    // TODO: check the into string pattern
    fn open(path: &str) -> Result<Self, Box<dyn ChromaError>>
    where
        Self: Sized;
    fn get(&self, key: BlockfileKey<K>) -> Result<&V, Box<dyn ChromaError>>;
    fn set(&mut self, key: BlockfileKey<K>, value: V) -> Result<(), Box<dyn ChromaError>>;
}

pub(crate) trait SplittableBlockFile<K: PartialEq + PartialOrd + Clone, V: BlockfileValue>:
    Blockfile<K, V>
{
}

struct HashMapBlockfile<K: PartialEq + PartialOrd + Clone, V> {
    map: std::collections::HashMap<BlockfileKey<K>, V>,
}

impl<K: PartialEq + PartialOrd + Hash + Clone, V: BlockfileValue> Blockfile<K, V>
    for HashMapBlockfile<K, V>
{
    // TODO: change this to respect path instead of ignoring it and creating a new thing
    fn open(_path: &str) -> Result<Self, Box<dyn ChromaError>> {
        Ok(HashMapBlockfile {
            map: std::collections::HashMap::new(),
        })
    }
    fn get(&self, key: BlockfileKey<K>) -> Result<&V, Box<dyn ChromaError>> {
        match self.map.get(&key) {
            Some(value) => Ok(value),
            None => {
                // TOOD: make error
                panic!("Key not found");
            }
        }
    }

    fn set(&mut self, key: BlockfileKey<K>, value: V) -> Result<(), Box<dyn ChromaError>> {
        self.map.insert(key, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockstore() {
        let mut blockfile = HashMapBlockfile::open("test").unwrap();
        let key = BlockfileKey {
            prefix: "text_prefix".to_string(),
            key: "key1".to_string(),
        };
        let _res = blockfile.set(key.clone(), "value1".to_string()).unwrap();
        let value = blockfile.get(key);
        // downcast to string
        assert_eq!(value.unwrap(), "value1");
    }
}