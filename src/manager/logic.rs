use crate::store::store::Store;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Manager {}

#[allow(warnings)]
impl Manager {
    pub fn run(store: Arc<Store>) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                for key in store.get_keys() {
                    match store.get_rem_time(&key) {
                        Ok(true) => {
                            store.delete(&key);
                        }
                        Ok(false) => {
                            //don't do anything
                        }
                        Err(e) => {
                            //key does not exist so continue
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
        thread::sleep(Duration::from_secs(2));
        Manager::run(arstore.clone());
        let cl_store = arstore.clone();
        match cl_store.get("hello".to_owned()) {
            Err(e) => {
                assert_eq!(1, 1)
            }
            Ok(v) => {
                eprintln!("{}", v);
                assert_eq!(1, 0)
            }
        }
    }
}
