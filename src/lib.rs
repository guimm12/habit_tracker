#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

#[wasm_bindgen]
pub fn create_item(id: u32, name: &str) -> JsValue {
    let item = Item {
        id,
        name: name.to_string(),
    };
    let serialized = serde_json::to_string(&item).unwrap();
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(&id.to_string(), &serialized)
        .unwrap();
    serde_wasm_bindgen::to_value(&item).unwrap()
}

#[wasm_bindgen]
pub fn read_item(id: u32) -> JsValue {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let item = storage.get_item(&id.to_string()).unwrap().unwrap();
    JsValue::from_str(&item)
}

#[wasm_bindgen]
pub fn update_item(id: u32, name: &str) -> JsValue {
    create_item(id, name)
}

#[wasm_bindgen]
pub fn delete_item(id: u32) {
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .remove_item(&id.to_string())
        .unwrap();
}
