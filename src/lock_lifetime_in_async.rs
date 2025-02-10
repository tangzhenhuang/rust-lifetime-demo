use std::{collections::HashMap, sync::{Arc, RwLock}};

async fn lock_lifetime() {
    let rw_lock = Arc::new(RwLock::new(HashMap::new()));

    let mut map = rw_lock.write().unwrap();
    if map.is_empty() {
        let value = some_async_function().await;
        map.insert(String::from("key1"), value);
    }
}

async fn some_async_function() -> String {
    //something calculate value async;
    String::from("value1")
}