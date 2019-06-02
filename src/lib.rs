use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;
use std::mem;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V> where K: Hash + Eq {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.empty() {
            self.rehash();
        }

        let bucket_idx = self.get_bucket_idx(&key);
        let bucket = &mut self.buckets[bucket_idx];

        // if the same key already exists, replace its old value with new value
        for (k, v) in bucket.iter_mut() {
            if *k == key {
                mem::replace(v, value);
                return None;
            }
        }

        self.items += 1;
        bucket.push((key, value));
        None
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let bucket_idx = self.get_bucket_idx(&key);

        self.buckets[bucket_idx]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn get_bucket_idx(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    fn rehash(&mut self) {
        let size = match self.buckets.len() {
            0 => 1,
            n => n * 2,
        };

        let mut new_buckets = Vec::with_capacity(size);
        new_buckets.extend((0..size).map(|_| Vec::new()));

        mem::replace(&mut self.buckets, new_buckets);
    }

    fn empty(&self) -> bool {
        self.items == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        // Create new HashMap
        let mut m = HashMap::new();
        // No item
        assert_eq!(m.items, 0);
        // Insert
        m.insert(1, 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get(1).unwrap(), 42);
        // Insert another value with existing key
        m.insert(1, 10);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get(1).unwrap(), 10);
        // Insert another value with new key
        m.insert(2, 20);
        assert_eq!(m.items, 2);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(m.get(100), None);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        m.insert("key".to_string(), 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key".to_string()).unwrap(), 42);
        m.insert("key".to_string(), 10);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        m.insert("key2".to_string(), 20);
        assert_eq!(m.items, 2);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(m.get("key100".to_string()), None);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        m.insert("key", 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key").unwrap(), 42);
        m.insert("key", 10);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key").unwrap(), 10);
        m.insert("key2", 20);
        assert_eq!(m.items, 2);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(m.get("key100"), None);
    }
}
