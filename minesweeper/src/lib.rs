mod minesweeper;
mod random;

use minesweeper::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
  static MINESWEEPER: RefCell<Minesweeper>
    = RefCell::new(Minesweeper::new(10, 10, 15));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
  MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
  MINESWEEPER.with(|ms| {
    ms.borrow_mut().open((x, y));
  });
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
  MINESWEEPER.with(|ms| {
    ms.borrow_mut().toggle_flag((x, y));
  });
}
