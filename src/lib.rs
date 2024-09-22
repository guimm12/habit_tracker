use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hellow from Rust!".to_string()
}
