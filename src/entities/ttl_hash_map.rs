use crate::entities::redis_element::RedisElement;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct TtlHashMap<K: Eq + Hash, V> {
    store: HashMap<K, V>,
    ttls: HashMap<K, SystemTime>,
    last_access: HashMap<K, SystemTime>,
}

impl<K: Clone + Eq + Hash, V> TtlHashMap<K, V> {
    pub fn new() -> Self {
        TtlHashMap {
            store: HashMap::new(),
            ttls: HashMap::new(),
            last_access: HashMap::new(),
        }
    }

    fn expired(&self, key: &K) -> bool {
        match self.ttls.get(key) {
            Some(ttl) => ttl.elapsed().is_ok(),
            None => false,
        }
    }

    /// Devuelve None si no existe la clave o expiró. Sino, devuelve el tiempo desde el anterior acceso.
    pub fn update_last_access(&mut self, key: &K) -> Option<Duration> {
        if !self.contains_key(&key) {
            return None;
        }
        match self.last_access.insert(key.clone(), SystemTime::now()) {
            Some(value) => Some(
                value
                    .duration_since(SystemTime::now())
                    .unwrap_or_else(|_| Duration::from_secs(0)),
            ),
            None => None,
        }
    }

    /// Devuelve None si no existe la clave, y SystemTime::UNIX_EPOCH si era persistente. Sino, devuelve el valor previo de ttl.
    pub fn set_ttl_relative(&mut self, key: K, duration: Duration) -> Option<SystemTime> {
        if !self.contains_key(&key) {
            return None;
        }
        let ttl = SystemTime::now() + duration;
        Some(self.ttls.insert(key, ttl).unwrap_or(SystemTime::UNIX_EPOCH))
    }

    /// Devuelve None si no existe la clave, y SystemTime::UNIX_EPOCH si era persistente. Sino, devuelve el valor previo de ttl.
    pub fn set_ttl_absolute(&mut self, key: K, ttl: SystemTime) -> Option<SystemTime> {
        if !self.contains_key(&key) {
            return None;
        }
        Some(self.ttls.insert(key, ttl).unwrap_or(SystemTime::UNIX_EPOCH))
    }

    pub fn delete_ttl(&mut self, key: &K) -> Option<SystemTime> {
        self.ttls.remove(key)
    }

    /// Devuelve None si no existe la clave, y una duración 0 si existe pero es persistente. Sino, devuelve el ttl.
    pub fn get_ttl(&mut self, key: &K) -> Option<Duration> {
        if !self.contains_key(key) {
            return None;
        }
        let ttl = match self.ttls.get(key) {
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
        self.update_last_access(&key);
        self.store.insert(key, value)
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.get(&key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.ttls.remove(key);
        self.last_access.remove(key);
        self.store.remove(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.expired(key) {
            self.remove(key);
            return None;
        }
        self.update_last_access(key);
        self.store.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.expired(key) {
            self.remove(key);
            return None;
        }
        self.update_last_access(key);
        self.store.get_mut(key)
    }

    /// Devuelve las keys sin chequear que no hayan expirado
    pub fn keys(&self) -> Keys<K, V> {
        self.store.keys()
    }
}

impl<K: Eq + Hash + fmt::Display, V: fmt::Display> TtlHashMap<K, V> {
    pub fn serialize(&self) -> String {
        let mut s = "".to_string();
        for (key, value) in self.store.iter() {
            let ttl = match self.ttls.get(key) {
                Some(t) => t
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::from_secs(0))
                    .as_secs(),
                None => 0,
            };
            s.push_str(format!("{},{},{}\n", key.to_string(), value.to_string(), ttl).as_str());
        }
        s
    }
}

impl TtlHashMap<String, RedisElement> {
    pub fn deserialize(s: String) -> Result<Self, String> {
        let mut map: TtlHashMap<String, RedisElement> = TtlHashMap::new();

        for element in s.lines() {
            let mut element = element.split(',');

            let key = element.next().ok_or("ERR syntax error")?;
            let value = element
                .next()
                .ok_or_else(|| format!("ERR missing value at key: {}", key))?;
            let ttl = element
                .next()
                .ok_or_else(|| format!("ERR missing ttl at key: {}", key))?
                .parse()
                .map_err(|_| format!("ERR ttl syntax error at key: {}", key))?;

            map.insert(key.to_string(), RedisElement::from(value));
            if ttl != 0 {
                map.set_ttl_absolute(
                    key.to_string(),
                    SystemTime::UNIX_EPOCH + Duration::from_secs(ttl),
                );
            }
        }
        Ok(map)
    }
}
