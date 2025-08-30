use std::hash::Hash;

use lru::DefaultHasher;

struct SizedValue<V: Sized> {
    value: V,
    byte_size: usize,
}

pub struct LruCache<K, V: Sized, S = DefaultHasher> {
    inner: lru::LruCache<K, SizedValue<V>, S>,
    byte_size: usize,
    max_byte_size: usize,
}

impl<K: Hash + Eq, V: Sized + Clone> LruCache<K, V> {
    pub fn new(max_byte_size: usize) -> Self {
        Self {
            inner: lru::LruCache::unbounded(),
            byte_size: 0,
            max_byte_size,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.inner.get(key).map(|v| &v.value)
    }

    pub fn push(&mut self, key: K, value: V, byte_size: usize) -> Option<(K, V)> {
        while byte_size + self.byte_size > self.max_byte_size && !self.inner.is_empty() {
            self.byte_size -= match self.inner.pop_lru() {
                Some((_, v)) => v.byte_size,
                None => 0,
            };
        }
        self.byte_size += byte_size;
        self.inner
            .push(key, SizedValue { value, byte_size })
            .map(|(k, v)| (k, v.value.clone()))
    }
}
