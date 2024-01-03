use ska_web::build_idx;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ska_map(file_name: &str) -> usize  {
    build_idx(file_name)
}
