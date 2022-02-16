use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello from Rust, {}!", name));
}

#[wasm_bindgen]
pub fn calculate(n: i32) -> i32 {
    return n * 2;
}
