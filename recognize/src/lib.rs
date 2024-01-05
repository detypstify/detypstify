mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(js_name = logHelloWorld)]
pub fn log_hello_world() {
    log("Hello, World!");
}

#[wasm_bindgen(js_name = getPowers)]
pub fn get_powers(n: u32) -> Vec<u32> {
    alert(&format!("Getting powers of {} ...", n));
    vec![n, n.pow(2), n.pow(3)]
}

#[wasm_bindgen(js_name = addTwoNumbers)]
pub fn add_two_numbers(a: u64, b: u64) -> u64 {
    a + b
}
