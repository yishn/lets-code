#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize) -> usize {
  let mut rng = thread_rng();

  rng.gen_range(min..max)
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = Math)]
  fn random() -> f64;
}

#[cfg(target_family = "wasm")]
pub fn random_range(min: usize, max: usize) -> usize {
  (random() * (max - min) as f64).floor() as usize + min
}
