use crate::store;
use std::borrow::Cow;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[allow(warnings)]
pub struct Connector {}

#[allow(warnings)]
impl Connector {
    pub fn run(store: Arc<store::store::Store>) {
        thread::spawn(move || {
            let listener = TcpListener::bind("localhost:4444").unwrap();
            eprintln!("Listening on port 44444");
            for _stream in listener.incoming() {
                let stream = _stream.unwrap();
                eprintln!("Connection establshed");
                let in_store = store.clone();
                thread::spawn(move || Self::handle_client(stream, in_store));
            }
        });
    }

    fn handle_client(mut stream: TcpStream, store: Arc<store::store::Store>) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                let iter = String::from_utf8_lossy(&buffer[..]);
                let mut response = Self::parse_string(iter, store.clone());
                response.push('\n');
                stream.write(response.as_bytes()).unwrap();
                true
            }
            Err(_) => {
                eprintln!("Closing stream");
                false
            }
        } {}
    }

    fn parse_string(req: Cow<str>, store: Arc<store::store::Store>) -> String {
        let mut iter = req.split_ascii_whitespace();
        match iter.next() {
            Some(val) => match val {
                "GET" => match iter.next() {
                    Some(val) => match store.get(String::from(val)) {
                        Ok(val) => val,
                        Err(e) => String::from("Error: Key does not exist"),
                    },
                    None => String::from("Error: Malfromed Input"),
                },
                "UPDATE" => {
                    let mut key = "";
                    let mut val = "";
                    match iter.next() {
                        Some(_key) => {
                            key = _key;
                        }
                        None => {
                            return String::from("Error: Malfromed Input");
                        }
                    }
                    match iter.next() {
                        Some(_val) => {
                            val = _val;
                        }
                        None => {
                            return String::from("Error: Malfromed Input");
                        }
                    }
                    match store.update(String::from(key), String::from(val)) {
                        Ok(val) => String::from("Success"),
                        Err(e) => String::from("Error: Key does not exist"),
                    }
                }
                "DELETE" => match iter.next() {
                    Some(val) => match store.delete(val) {
                        Ok(val) => val,
                        Err(e) => String::from("Error: Key does not exist"),
                    },
                    None => String::from("Error: Malfromed Input"),
                },
                "INSERT" => {
                    let mut key = "";
                    let mut val = "";
                    match iter.next() {
                        Some(_key) => {
                            key = _key;
                        }
                        None => {
                            return String::from("Error: Malfromed Input");
                        }
                    }
                    match iter.next() {
                        Some(_val) => {
                            val = _val;
                        }
                        None => {
                            return String::from("Error: Malfromed Input");
                        }
                    }
                    match store.insert(String::from(key), String::from(val), 40) {
                        Ok(val) => String::from("Success"),
                        Err(e) => String::from("Error: Key does not exist"),
                    }
                }
                _ => String::from("Error: Unknown Method"),
            },
            _ => String::from("Error: Malfromed Input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connector_test() {
        let store = Arc::new(store::store::Store::new());
        Connector::run(store.clone());
        assert_eq!(1, 1);
    }

    #[test]
    fn parse_test_insert() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("INSERT hello world");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Success"));
    }

    #[test]
    fn parse_test_update() {
        let store = Arc::new(store::store::Store::new());
        store.insert(String::from("hello"), String::from("world"), 40);
        let request = Cow::from("UPDATE hello corona");
        let response = Connector::parse_string(request, store.clone());
        let request = Cow::from("GET hello");
        let response = Connector::parse_string(request, store.clone());
        assert_eq!(response, String::from("corona"));
    }

    #[test]
    fn parse_test_insert_bad_input() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("INSERT hello");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Error: Malfromed Input"));
    }

    #[test]
    fn parse_test_unknown_method() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("CRUD hello world");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Error: Unknown Method"));
    }

    #[test]
    fn parse_test_update_bad_input() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("UPDATE hello");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Error: Malfromed Input"));
    }

    #[test]
    fn parse_test_get_unknown_key() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("GET hello");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Error: Key does not exist"));
    }

    #[test]
    fn empty_input() {
        let store = Arc::new(store::store::Store::new());
        let request = Cow::from("");
        let response = Connector::parse_string(request, store);
        assert_eq!(response, String::from("Error: Malfromed Input"));
    }
}
