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

    /// Actualiza el último acceso a la clave. Devuelve el tiempo transcurrido desde el anterior acceso. Devuelve None si no existe la clave o expiró.
    pub fn update_last_access(&mut self, key: &K) -> Option<Duration> {
        if !self.contains_key(&key) {
            return None;
        }

        self.last_access
            .insert(key.clone(), SystemTime::now())
            .map(|value| value.elapsed().unwrap_or_else(|_| Duration::from_secs(0)))
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

    /// Elimina el time-to-live de la clave, devolviendo el timestamp de expiración. Si no tenía una expiración, devuelve None.
    pub fn delete_ttl(&mut self, key: &K) -> Option<SystemTime> {
        if self.expired(key) {
            self.remove(key);
            return None;
        }
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

    /// Devuelve la cantidad de claves guardadas, sin chequear que no hayan expirado.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.remove(&key);
        self.last_access.insert(key.clone(), SystemTime::now());
        self.store.insert(key, value)
    }

    /// Devuelve si la clave existe o no, chequeando que no haya expirado. Si expiró, la borra.
    pub fn contains_key(&mut self, key: &K) -> bool {
        match self.store.get(key) {
            Some(_value) => {
                if self.expired(key) {
                    self.remove(key);
                    return false;
                }
                true
            }
            None => false,
        }
    }

    /// Elimina el par clave-valor, devolviendo ese valor. Si no existía la clave, devuelve None.
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

    /// Devuelve las claves, sin chequear que no hayan expirado
    pub fn keys(&self) -> Keys<K, V> {
        self.store.keys()
    }
}

impl TtlHashMap<String, RedisElement> {
    /// Devuelve un string con el TtlHashMap serializado. Se guardan todos los key-value con su ttl (como Unix Timestamp en segundos).
    pub fn serialize(&self) -> Vec<u8> {
        let OP_EOF: u8 = 0xff;
        let OP_EXPIRETIME = 0xfd;
        let OP_RESIZEDB = 0xfb;

        let mut s: Vec<u8> = vec![OP_RESIZEDB];
        s.append(&mut TtlHashMap::length_encode(self.store.len()));
        s.append(&mut TtlHashMap::length_encode(self.ttls.len()));

        for (key, value) in self.store.iter() {
            if let Some(ttl) = self.ttls.get(key) {
                let secs = ttl
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::from_secs(0))
                    .as_secs();
                s.push(OP_EXPIRETIME);
                s.append(&mut secs.to_be_bytes().to_vec()); //revisar LITTLE ENDIAN O BIG ENDIAN
            }
            s.push(TtlHashMap::get_value_type(value));
            s.append(&mut key.as_bytes().to_vec());
            s.append(&mut value.to_string().as_bytes().to_vec());
        }

        s.push(OP_EOF);
        s
    }

    // Deserializa un vector de bytes para devolver un TtlHashMap cargado con todos los RedisElemets.
    pub fn deserialize(s: Vec<u8>) -> std::io::Result<Self> {
        let mut map: TtlHashMap<String, RedisElement> = TtlHashMap::new();

        match s.split_off(1) {
            OP_RESIZEDB => {
                map.set_size(
                    TtlHashMap::length_decode(&mut s) as usize,
                    TtlHashMap::length_decode(&mut s) as usize,
                );
                map.load(s);
                Ok(map)
            }
            OP_EOF => Ok(map),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "OP code unknown.",
            ))?, //CORREGIR ERROR
        }
    }

    fn length_decode(length: &mut Vec<u8>) -> u32 {
        let first_byte = length.split_off(1).first().unwrap(); //unwrap! puede fallar? no deberia
        match first_byte >> 6 {
            0b00 => *first_byte as u32,
            0b01 => (first_byte & 0b00111111) as u32,
            0b10 => TtlHashMap::as_u32_be(&length.split_off(4)),
            0b11 => 0, //caso no implementado en encode! agregarlo
        }
    }

    fn as_u32_be(array: &[u8]) -> u32 {
        ((array[0] as u32) << 24)
            + ((array[1] as u32) << 16)
            + ((array[2] as u32) << 8)
            + ((array[3] as u32) << 0)
    }

    fn set_size(&mut self, store_size: usize, ttl_size: usize) {
        self.store.reserve(store_size);
        self.ttls.reserve(ttl_size);
    }

    fn load(&mut self, s: Vec<u8>) {
            // USAR ITERADORES
        while s.split_off(1).first().unwrap() != OP_EOF //unwrap!
            match s.split_off(1) {
                OP_EXPIRETIME => {
                    
                }
                _ => { //persistente
                }
            }
    }

    fn length_encode(length: usize) -> Vec<u8> {
        if length < 64 {
            // 00 + length in 6 bits
            vec![length as u8]
        } else if length < 16384 {
            // 01 + length in 14 bits
            vec![0x40 | (length >> 8) as u8, length as u8]
        } else {
            // 1000 0000 + length in 32 bits
            vec![
                0x80,
                (length >> 24) as u8,
                (length >> 16) as u8,
                (length >> 8) as u8,
                length as u8,
            ]
        }
        //if length > 0xffffffff ?
    }

    fn get_value_type(value: &RedisElement) -> u8 {
        match value {
            RedisElement::String(_) => 0,
            RedisElement::List(_) => 1,
            RedisElement::Set(_) => 2,
            _ => 3,
        }
    }
}

#[allow(unused_imports)]
mod test {
    use crate::entities::ttl_hash_map::RedisElement;
    use crate::entities::ttl_hash_map::TtlHashMap;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_get_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);

        assert_eq!(map.get(&key).unwrap(), &1);
    }

    #[test]
    fn test_get_mut_key_modifies_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        let value = map.get_mut(&key).unwrap();
        *value += 1;

        assert_eq!(map.get(&key).unwrap(), &2);
    }

    #[ignore]
    #[test]
    fn test_ttl_relative_deletes_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        map.set_ttl_relative(key.clone(), Duration::from_secs(1));

        std::thread::sleep(Duration::from_secs(1));

        assert!(map.get(&key).is_none());
    }

    #[test]
    fn test_ttl_absolute_deletes_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        map.set_ttl_absolute(key.clone(), SystemTime::now());

        assert!(map.get(&key).is_none());
    }

    #[test]
    fn test_contains_key_on_expired_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        map.set_ttl_absolute(key.clone(), SystemTime::now());

        assert!(!map.contains_key(&key));
    }

    #[test]
    fn test_delete_ttl_on_expired_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        map.set_ttl_absolute(key.clone(), SystemTime::now());

        assert!(map.delete_ttl(&key).is_none());
        assert!(map.get(&key).is_none());
    }

    #[test]
    fn test_delete_ttl_on_presistent_key() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);

        assert!(map.delete_ttl(&key).is_none());
        assert!(map.get(&key).is_some());
    }

    #[test]
    fn test_insert_key_twice_deletes_ttl() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        map.set_ttl_absolute(key.clone(), SystemTime::now());
        map.insert(key.clone(), 1);

        assert!(map.delete_ttl(&key).is_none());
        assert!(map.get(&key).is_some());
    }

    #[ignore]
    #[test]
    fn test_new_key_last_access() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        std::thread::sleep(Duration::from_secs(1));

        assert_eq!(map.update_last_access(&key).unwrap().as_secs(), 1);
    }

    #[ignore]
    #[test]
    fn test_get_changes_last_access() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), 1);
        std::thread::sleep(Duration::from_secs(1));
        map.get(&key);

        assert_eq!(map.update_last_access(&key).unwrap().as_secs(), 0);
    }

    #[test]
    fn test_remove_key_and_add_again() {
        let mut map: TtlHashMap<String, u8> = TtlHashMap::new();
        let key = "key".to_string();

        assert!(map.remove(&key).is_none());
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let mut map: TtlHashMap<String, RedisElement> = TtlHashMap::new();
        let key = "key".to_string();

        map.insert(key.clone(), RedisElement::String("value".to_string()));
        map.set_ttl_relative(key.clone(), Duration::from_secs(1));

        let ttl = (SystemTime::now() + Duration::from_secs(1))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let serialization = map.serialize();
        assert_eq!(serialization, "key,value,".to_owned() + &ttl + "\n");

        let mut new_map = TtlHashMap::deserialize(serialization).unwrap();
        assert_eq!(new_map.get(&key).unwrap().to_string(), "value");
    }
}
