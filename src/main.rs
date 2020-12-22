mod manager;
mod store;
mod tcpconnector;
use std::sync::Arc;

fn main() {
    let store = Arc::new(store::store::Store::new());
    manager::logic::Manager::run(store.clone());
}
