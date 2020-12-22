use crate::store;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[allow(warnings)]
pub struct Connector {}

#[allow(warnings)]
impl Connector {
    fn run(store: Arc<store::store::Store>) {
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:80").unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                Self::handle_client(stream, store.clone());
            }
        });
    }
    fn handle_client(mut stream: TcpStream, store: Arc<store::store::Store>) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let iter = String::from_utf8_lossy(&buffer[..]);
        let mut iter = iter.split_ascii_whitespace();
        match iter.next() {
            Some(val) => match val {
                "GET" => match iter.next() {
                    Some(val) => match store.get(String::from(val)) {
                        Ok(val) => {
                            stream.write(val.as_bytes()).unwrap();
                        }
                        Err(e) => {
                            stream.write("Error: Key does not exist".as_bytes());
                        }
                    },
                    None => {
                        stream.write("Error: malformed input".as_bytes());
                    }
                },
                "UPDATE" => {
                    let mut key = "";
                    let mut val = "";
                    match iter.next() {
                        Some(_key) => {
                            key = _key;
                        }
                        None => {
                            stream.write("Error: malformed input".as_bytes());
                        }
                    }
                    match iter.next() {
                        Some(_val) => {
                            val = _val;
                        }
                        None => {
                            stream.write("Error: malformed input".as_bytes());
                        }
                    }
                    match store.update(String::from(key), String::from(val)) {
                        Ok(val) => {
                            stream.write("Success".as_bytes()).unwrap();
                        }
                        Err(e) => {
                            stream.write("Error: Key does not exist".as_bytes());
                        }
                    }
                }
                "DELETE" => match iter.next() {
                    Some(val) => match store.delete(val) {
                        Ok(val) => {
                            stream.write(val.as_bytes()).unwrap();
                        }
                        Err(e) => {
                            stream.write("Error: Key does not exist".as_bytes());
                        }
                    },
                    None => {
                        stream.write("Error: malformed input".as_bytes());
                    }
                },
                "INSERT" => {
                    let mut key = "";
                    let mut val = "";
                    match iter.next() {
                        Some(_key) => {
                            key = _key;
                        }
                        None => {
                            stream.write("Error: malformed input".as_bytes());
                        }
                    }
                    match iter.next() {
                        Some(_val) => {
                            val = _val;
                        }
                        None => {
                            stream.write("Error: malformed input".as_bytes());
                        }
                    }
                    match store.insert(String::from(key), String::from(val), 40) {
                        Ok(val) => {
                            stream.write("Success".as_bytes()).unwrap();
                        }
                        Err(e) => {
                            stream.write("Error: Key does not exist".as_bytes());
                        }
                    }
                }
                _ => {
                    stream.write("Error: malformed input".as_bytes());
                }
            },
            _ => {
                stream.write("Error: malformed input".as_bytes());
            }
        }
        stream.flush().unwrap();
    }
}

#[cfg(test)] 
mod tests{
    use super::*;
    #[test]
    fn connector_test() {
        let store = Arc::new(store::store::Store::new());
        Connector::run(store.clone());
        assert_eq!(1, 1);
    }
}


