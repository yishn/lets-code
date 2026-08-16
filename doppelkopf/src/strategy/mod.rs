mod ai;
mod describe;
mod human;

pub use ai::*;
pub use human::*;
pub use describe::*;

use crate::core::{Action, Game};
use anyhow::Result;
use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GenerateDispatchError {
  GameEnded,
  InvalidActionGenerated,
}

impl Display for GenerateDispatchError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self, f)
  }
}

impl Error for GenerateDispatchError {}

pub trait Strategy {
  async fn generate_dispatch(&self, game: &Game) -> Result<Action>;
}
