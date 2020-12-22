mod manager;
mod store;
use std::sync::Arc;

fn main() {
    let store = Arc::new(store::store::Store::new());
}
