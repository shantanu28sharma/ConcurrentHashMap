use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

enum Error {
    NotFound,
}

pub struct Store {
    data: RwLock<HashMap<String, Info>>,
}

pub struct Info{
    val: String,
    now: Instant,
    elapse: u64
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: RwLock::new(HashMap::new()),
        }
    }
    pub fn insert(&mut self, key: String, val: String, expire: u64) {
        let mut map = self.data.write().unwrap();
        let data = Info{
            val: val,
            now: Instant::now(),
            elapse: expire
        };
        map.insert(key, data);
    }
    pub fn delete(&mut self, key: String) -> Result<String, Error> {
        let mut map = self.data.write().unwrap();
        match map.remove(&key) {
            Some(val) => Ok(val.val[..].to_string()),
            None => Err(Error::NotFound),
        }
    }
    pub fn get(&mut self, key: String) -> Result<String, Error> {
        let map = self.data.read().unwrap();
        match map.get(&key) {
            Some(val) => Ok(val.val[..].to_string()),
            None => Err(Error::NotFound),
        }
    }
    pub fn update(&mut self, key: String, val: String) -> Result<(), Error> {
        let mut map = self.data.write().unwrap();
        match map.get(&key){
            Some(_val) => {
                let expiry = _val.elapse;
                let instant = _val.now;
                let data = Info{
                    val: val,
                    now: instant,
                    elapse: expiry
                };
                map.insert(key, data);
                Ok(())
            },
            None => {Err(Error::NotFound)}
        }
    }
    pub fn get_rem_time(&self, key: String)-> Result<u64, Error>{
        let map = self.data.read().unwrap();
        match map.get(&key) {
            Some(val) => Ok(val.now.elapsed().as_secs()-val.elapse),
            None => Err(Error::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_map() {
        let mut store = Store::new();
        store.insert(String::from("hello"), String::from("world"), 40);
        let res = store.get(String::from("hello"));
        match res {
            Ok(v) => assert_eq!(v, String::from("world")),
            Err(e) => assert_eq!(0, 1),
        }
    }
    #[test]
    fn update_map() {
        let mut store = Store::new();
        store.insert(String::from("hello"), String::from("world"), 40);
        match store.update(String::from("hello"), String::from("corona")) {
            Ok(()) => match store.get(String::from("hello")) {
                Ok(val) => assert_eq!(val, String::from("corona")),
                Err(e) => assert_eq!(0, 1),
            },
            Err(e) => {
                assert_eq!(1, 0);
            }
        }
    }
}