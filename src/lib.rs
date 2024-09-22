use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from Rust!".to_string()
}

#[wasm_bindgen]
pub fn hello_world() -> String {
    "Hello World!".to_string()
}
