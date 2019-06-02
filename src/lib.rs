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
        // rehash the HashMap in these cases
        //   * HashMap has no buckets (Just after HashMap::new())
        //   * HashMap has items, the number of which is over 75% of the number of buckets
        if self.empty() || self.items > (3 * self.buckets.len() / 4) {
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

    pub fn remove(&mut self, key: K) -> Option<V> {
        let bucket_idx = self.get_bucket_idx(&key);
        let bucket = &mut self.buckets[bucket_idx];
        let idx = bucket
            .iter()
            .position(|(k, _)| *k == key)?;

        self.items -= 1;
        bucket.swap_remove(idx);
        None
    }

    pub fn get(&self, key: K) -> Option<&V> {
        if self.empty() {
            return None;
        }

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

        // set again existing key-value pairs to new buckets
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let idx = (hasher.finish() % size as u64) as usize;
            new_buckets[idx].push((key, value))
        }

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
        assert_eq!(m.buckets.len(), 0);
        assert_eq!(m.get(100), None);
        // Insert
        m.insert(1, 42);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 1);
        assert_eq!(*m.get(1).unwrap(), 42);
        assert_eq!(m.get(100), None);
        // Insert another value with existing key
        m.insert(1, 10);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(m.get(100), None);
        // Insert another value with new key
        m.insert(2, 20);
        assert_eq!(m.items, 2);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(m.get(100), None);
        // Insert another value with new key
        m.insert(3, 30);
        assert_eq!(m.items, 3);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(*m.get(3).unwrap(), 30);
        assert_eq!(m.get(100), None);
        // Insert another value with new key
        m.insert(4, 40);
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(*m.get(3).unwrap(), 30);
        assert_eq!(*m.get(4).unwrap(), 40);
        assert_eq!(m.get(100), None);
        // Insert another value with new key
        m.insert(5, 50);
        assert_eq!(m.items, 5);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(*m.get(3).unwrap(), 30);
        assert_eq!(*m.get(4).unwrap(), 40);
        assert_eq!(*m.get(5).unwrap(), 50);
        assert_eq!(m.get(100), None);
        // Remove value by key
        m.remove(3);
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get(1).unwrap(), 10);
        assert_eq!(*m.get(2).unwrap(), 20);
        assert_eq!(*m.get(4).unwrap(), 40);
        assert_eq!(*m.get(5).unwrap(), 50);
        assert_eq!(m.get(3), None);
        assert_eq!(m.get(100), None);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        assert_eq!(m.buckets.len(), 0);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key".to_string(), 42);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 1);
        assert_eq!(*m.get("key".to_string()).unwrap(), 42);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key".to_string(), 10);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key2".to_string(), 20);
        assert_eq!(m.items, 2);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key3".to_string(), 30);
        assert_eq!(m.items, 3);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(*m.get("key3".to_string()).unwrap(), 30);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key4".to_string(), 40);
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(*m.get("key3".to_string()).unwrap(), 30);
        assert_eq!(*m.get("key4".to_string()).unwrap(), 40);
        assert_eq!(m.get("key100".to_string()), None);
        m.insert("key5".to_string(), 50);
        assert_eq!(m.items, 5);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(*m.get("key3".to_string()).unwrap(), 30);
        assert_eq!(*m.get("key4".to_string()).unwrap(), 40);
        assert_eq!(*m.get("key5".to_string()).unwrap(), 50);
        assert_eq!(m.get("key100".to_string()), None);
        m.remove("key3".to_string());
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get("key".to_string()).unwrap(), 10);
        assert_eq!(*m.get("key2".to_string()).unwrap(), 20);
        assert_eq!(*m.get("key4".to_string()).unwrap(), 40);
        assert_eq!(*m.get("key5".to_string()).unwrap(), 50);
        assert_eq!(m.get("key3".to_string()), None);
        assert_eq!(m.get("key100".to_string()), None);

        let mut m = HashMap::new();
        assert_eq!(m.items, 0);
        assert_eq!(m.buckets.len(), 0);
        assert_eq!(m.get("key100"), None);
        m.insert("key", 42);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 1);
        assert_eq!(*m.get("key").unwrap(), 42);
        assert_eq!(m.get("key100"), None);
        m.insert("key", 10);
        assert_eq!(m.items, 1);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(m.get("key100"), None);
        m.insert("key2", 20);
        assert_eq!(m.items, 2);
        assert_eq!(m.buckets.len(), 2);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(m.get("key100"), None);
        m.insert("key3", 30);
        assert_eq!(m.items, 3);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(*m.get("key3").unwrap(), 30);
        assert_eq!(m.get("key100"), None);
        m.insert("key4", 40);
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 4);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(*m.get("key3").unwrap(), 30);
        assert_eq!(*m.get("key4").unwrap(), 40);
        assert_eq!(m.get("key100"), None);
        m.insert("key5", 50);
        assert_eq!(m.items, 5);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(*m.get("key3").unwrap(), 30);
        assert_eq!(*m.get("key4").unwrap(), 40);
        assert_eq!(*m.get("key5").unwrap(), 50);
        assert_eq!(m.get("key100"), None);
        m.remove("key3");
        assert_eq!(m.items, 4);
        assert_eq!(m.buckets.len(), 8);
        assert_eq!(*m.get("key").unwrap(), 10);
        assert_eq!(*m.get("key2").unwrap(), 20);
        assert_eq!(*m.get("key4").unwrap(), 40);
        assert_eq!(*m.get("key5").unwrap(), 50);
        assert_eq!(m.get("key3"), None);
        assert_eq!(m.get("key100"), None);
    }
}
