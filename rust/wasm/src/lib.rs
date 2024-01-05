use ska_web::*;

use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct SkaRef {
    k: usize,
    kmers: Vec<u128>,
    count: usize,
}

#[wasm_bindgen]
impl SkaRef {
    pub fn new(ref_file: &str) -> Self {
        init_panic_hook();

        let idx = idx_ref(ref_file);
        log(&format!("Indexed reference with return value {}", idx));

        // Placeholder
        let k = 31;
        let size = 3000000;
        let kmers = vec![0; size];
        Self {k, kmers, count: 0}
    }

    pub fn map(&mut self, input_file: &str) -> Vec<usize> {
        let idx = map_to_ref(input_file);
        let mut result: f64 = 0.0;
        // Take some time
        for i in 0..50000000 {
            result += ((i % 201) * (i % 7)) as f64;
        }
        self.count += 1;
        log(&format!("Mapped file {} with return value {}", self.count, idx));
        vec![1; 10]
    }
}
