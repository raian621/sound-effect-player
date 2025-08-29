use std::hash::Hash;

use lru::DefaultHasher;

pub struct LruCache<K, V: Sized, S = DefaultHasher> {
    inner: lru::LruCache<K, V, S>,
    byte_size: usize,
    max_byte_size: usize,
}

impl<K: Hash + Eq, V: Sized> LruCache<K, V> {
    pub fn new(max_byte_size: usize) -> Self {
        Self {
            inner: lru::LruCache::unbounded(),
            byte_size: 0,
            max_byte_size,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn push(&mut self, key: K, value: V) -> Option<(K, V)> {
        let val_size = std::mem::size_of_val(&value);
        while val_size + self.byte_size > self.max_byte_size {
            self.byte_size -= match self.inner.pop_lru() {
                Some(v) => std::mem::size_of_val(&v),
                None => 0,
            };
        }
        self.inner.push(key, value)
    }
}
