use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = Math)]
  fn random() -> f32;
}

fn random_usize(min: usize, max: usize) -> usize {
  (random() * ((max - min) as f32) + min as f32).floor() as usize
}

pub type Vertex = (usize, usize);

pub struct VertexInfo {
  pub has_mine: bool,
  pub flagged: bool,
  pub open: bool,
}

pub enum OpenResult {
  Mine,
  NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
  width: usize,
  height: usize,
  mines: HashSet<Vertex>,
  flags: HashSet<Vertex>,
  open_vertices: HashSet<Vertex>,
  lost: bool,
}

impl Minesweeper {
  pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
    let mut mines = HashSet::new();

    while mines.len() < mine_count {
      mines.insert((random_usize(0, width), random_usize(0, height)));
    }

    Minesweeper {
      width,
      height,
      mines,
      flags: HashSet::new(),
      open_vertices: HashSet::new(),
      lost: false,
    }
  }

  pub fn has(&self, (x, y): Vertex) -> bool {
    x < self.width || y < self.height
  }

  pub fn get(&self, vertex: Vertex) -> Option<VertexInfo> {
    if !self.has(vertex) {
      None
    } else {
      Some(VertexInfo {
        has_mine: self.mines.contains(&vertex),
        flagged: self.flags.contains(&vertex),
        open: self.open_vertices.contains(&vertex),
      })
    }
  }

  pub fn neighbors(&self, (x, y): Vertex) -> impl Iterator<Item = Vertex> {
    let width = self.width;
    let height = self.height;

    (x.max(1) - 1..=(x + 1).min(width - 1)).flat_map(move |i| {
      (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j))
    })
  }

  pub fn peek(&self, vertex: Vertex) -> Option<OpenResult> {
    let info = self.get(vertex);

    info.map(|info| {
      if info.has_mine {
        OpenResult::Mine
      } else {
        OpenResult::NoMine(
          self
            .neighbors(vertex)
            .map(|n| self.get(n).unwrap().has_mine)
            .map(|has_mine| if has_mine { 1 } else { 0 })
            .sum(),
        )
      }
    })
  }

  pub fn open(&mut self, vertex: Vertex) -> Option<OpenResult> {
    let info = self.get(vertex);
    let open = info.as_ref().map(|info| info.open).unwrap_or(false);
    let flagged = info.as_ref().map(|info| info.flagged).unwrap_or(true);

    if self.lost || flagged {
      return None;
    }

    let result = self.peek(vertex);

    if result.is_some() {
      self.open_vertices.insert(vertex);
    }

    match result {
      Some(OpenResult::Mine) => {
        self.lost = true;
      }
      Some(OpenResult::NoMine(0)) => {
        // Open all neighboring fields

        for neighbor in self.neighbors(vertex) {
          if !self.get(neighbor).unwrap().open {
            self.open(neighbor);
          }
        }
      }
      Some(OpenResult::NoMine(mines)) if open => {
        let flagged_neighbor_count = self
          .neighbors(vertex)
          .map(|neighbor| self.get(neighbor).unwrap())
          .filter(|info| info.flagged && !info.open)
          .count() as u8;

        if flagged_neighbor_count == mines {
          // Open up all neighboring fields

          for neighbor in self.neighbors(vertex) {
            if !self.get(neighbor).unwrap().open {
              self.open(neighbor);
            }
          }
        }
      }
      _ => {}
    }

    result
  }

  pub fn flag(&mut self, vertex: Vertex) -> bool {
    if self.lost || self.get(vertex).map(|info| info.open).unwrap_or(true) {
      return false;
    }

    if self.has(vertex) {
      self.flags.insert(vertex);
      true
    } else {
      false
    }
  }

  pub fn unflag(&mut self, vertex: Vertex) -> bool {
    if self.lost {
      return false;
    }

    if self.has(vertex) {
      self.flags.remove(&vertex);
      true
    } else {
      false
    }
  }

  pub fn print(&self) -> String {
    (0..self.height)
      .map(|y| {
        (0..self.width)
          .map(move |x| (x, y))
          .map(|vertex| (vertex, self.get(vertex).unwrap()))
          .map(|(vertex, info)| {
            if !self.lost {
              if info.flagged {
                "🚩".to_string()
              } else if info.open {
                match self.peek(vertex).unwrap() {
                  OpenResult::Mine => "💣".to_string(),
                  OpenResult::NoMine(mines) => {
                    if mines > 0 {
                      format!(" {}", mines)
                    } else {
                      "⬜".to_string()
                    }
                  }
                }
              } else {
                "🟪".to_string()
              }
            } else {
              match self.peek(vertex).unwrap() {
                OpenResult::Mine => "💣".to_string(),
                OpenResult::NoMine(mines) => {
                  if info.open {
                    if mines > 0 {
                      format!(" {}", mines)
                    } else {
                      "⬜".to_string()
                    }
                  } else {
                    "🟪".to_string()
                  }
                }
              }
            }
          })
          .fold(String::new(), |mut acc, ch| {
            acc.push_str(&ch);
            acc.push(' ');
            acc
          })
      })
      .fold(String::new(), |mut acc, row| {
        acc.push_str(&row);
        acc.push('\n');
        acc
      })
  }
}
