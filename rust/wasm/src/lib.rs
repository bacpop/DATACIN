use ska_web::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
        self.count += 1;
        log(&format!("Mapped file {} with return value {}", self.count, idx));
        vec![1; 10]
    }
}
