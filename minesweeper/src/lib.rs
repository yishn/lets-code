mod minesweeper;

use minesweeper::Minesweeper;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
  static MINESWEEPER: RefCell<Minesweeper> =
    RefCell::new(Minesweeper::new(10, 10, 30));
}

#[wasm_bindgen(module = "/render.js")]
extern "C" {
  fn render(data: &str);
}

fn render_minesweeper(minesweeper: &Minesweeper) {
  render(&minesweeper.print())
}

#[wasm_bindgen(start)]
pub fn main() {
  MINESWEEPER.with(|ms| {
    render_minesweeper(&ms.borrow());
  });
}

#[wasm_bindgen]
pub fn open(x: usize, y: usize) {
  MINESWEEPER.with(|ms| {
    ms.borrow_mut().open((x, y));
    render_minesweeper(&ms.borrow());
  });
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
  MINESWEEPER.with(|ms| {
    let flagged = ms
      .borrow()
      .get((x, y))
      .map(|info| info.flagged)
      .unwrap_or(false);

    if !flagged {
      ms.borrow_mut().flag((x, y));
    } else {
      ms.borrow_mut().unflag((x, y));
    }

    render_minesweeper(&ms.borrow());
  });
}
