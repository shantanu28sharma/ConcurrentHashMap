use crate::store::store::Store;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Manager {}

impl Manager {
    pub fn run(store: Arc<Store>) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                let keys = store.get_keys();
                for key in keys {
                    match store.get_rem_time(&key) {
                        Ok(true) => {
                            store.delete(&key);
                        }
                        Ok(false) => {
                            //don't do anything
                        }
                        Err(e) => {
                            continue;
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn check_deletion() {
        let par_store = Store::new();
        par_store.insert(String::from("hello"), String::from("World"), 1);
        let arstore = Arc::new(par_store);
        Manager::run(arstore.clone());
        let cl_store = arstore.clone();
        match cl_store.get("hello".to_owned()) {
            Err(e) => {assert_eq!(1, 1)},
            Ok(v) => {assert_eq!(1, 0)}
        }
    }
}
