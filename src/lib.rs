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
        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        m.insert(1, 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get(1).unwrap(), 42);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        m.insert("key".to_string(), 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key".to_string()).unwrap(), 42);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        m.insert("key", 42);
        assert_eq!(m.items, 1);
        assert_eq!(*m.get("key").unwrap(), 42);
    }
}
