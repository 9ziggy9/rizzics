mod la;
use la::{Matrix3x3, Vector3};

use wasm_bindgen::prelude::*;

// This function will be callable from JavaScript
#[wasm_bindgen]
pub fn greet() {
    let xs: Matrix3x3 = Matrix3x3::new(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0
    );
    alert("Hello, wasm!");
    log(&xs.to_string());
}

// A simple function to demonstrate using Rust from JavaScript.
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
