use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = Math)]
  fn random() -> f64;
}

pub fn random_range(min: usize, max: usize) -> usize {
  (random() * (max - min) as f64).floor() as usize + min
}
