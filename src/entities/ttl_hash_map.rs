use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct TtlHashMap<K: Eq + Hash, V> {
    store: HashMap<K, V>,
    timestamps: HashMap<K, SystemTime>,
}

impl<K: Eq + Hash, V> TtlHashMap<K, V> {
    pub fn new() -> Self {
        TtlHashMap {
            store: HashMap::new(),
            timestamps: HashMap::new(),
        }
    }

    fn expired(&self, key: &K) -> bool {
        match self.timestamps.get(key) {
            Some(ttl) => ttl.elapsed().is_ok(),
            None => false,
        }
    }

    /// Devuelve None si no existe la clave, y SystemTime::UNIX_EPOCH si era persistente. Sino, devuelve el valor previo de ttl.
    pub fn set_ttl_relative(&mut self, key: K, duration: Duration) -> Option<SystemTime> {
        if !self.contains_key(&key) {
            return None;
        }
        let ttl = SystemTime::now() + duration;
        Some(
            self.timestamps
                .insert(key, ttl)
                .unwrap_or(SystemTime::UNIX_EPOCH),
        )
    }

    /// Devuelve None si no existe la clave, y SystemTime::UNIX_EPOCH si era persistente. Sino, devuelve el valor previo de ttl.
    pub fn set_ttl_absolute(&mut self, key: K, ttl: SystemTime) -> Option<SystemTime> {
        if !self.contains_key(&key) {
            return None;
        }
        Some(
            self.timestamps
                .insert(key, ttl)
                .unwrap_or(SystemTime::UNIX_EPOCH),
        )
    }

    pub fn delete_ttl(&mut self, key: &K) -> Option<SystemTime> {
        self.timestamps.remove(key)
    }

    /// Devuelve None si no existe la clave, y una duración 0 si existe pero es persistente. Sino, devuelve el ttl.
    pub fn get_ttl(&mut self, key: &K) -> Option<Duration> {
        if !self.contains_key(key) {
            return None;
        }
        let ttl = match self.timestamps.get(key) {
            Some(value) => value
                .duration_since(SystemTime::now())
                .unwrap_or_else(|_| Duration::from_secs(0)),
            None => Duration::from_secs(0),
        };
        Some(ttl)
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.delete_ttl(&key);
        self.store.insert(key, value)
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.get(&key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.expired(key) {
            self.remove(key);
            return None;
        }

        self.store.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.expired(key) {
            self.remove(key);
            return None;
        }

        self.store.get_mut(key)
    }
}
