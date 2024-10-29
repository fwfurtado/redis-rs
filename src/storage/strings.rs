use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref SET_STORAGE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


pub fn set(key: String, value: String) {
    if let Ok(mut map) = SET_STORAGE.lock() {
        map.insert(key, value);
    }
}

pub fn get(key: &str) -> Option<String> {
    if let Ok(map) = SET_STORAGE.lock() {
        map.get(key).map(|v| v.to_string())
    } else {
        None
    }
}

pub fn m_set(values: Vec<(String, String)>) {
    if let Ok(mut map) = SET_STORAGE.lock() {
        for (k, v) in values {
            map.insert(k, v);
        }
    }
}

pub fn m_get(keys: Vec<String>) -> Vec<Option<String>> {
    if let Ok(_map) = SET_STORAGE.lock() {
        keys.iter().map(|k| _map.get(k).map( |value| value.clone())).collect()
    } else {
        vec![]
    }
}