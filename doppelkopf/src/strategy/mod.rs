mod ai;
mod describe;
mod human;
mod monte_carlo;
mod random;

pub use ai::*;
pub use describe::*;
pub use human::*;
pub use monte_carlo::*;
pub use random::*;

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
  async fn generate_dispatch(&mut self, game: &Game) -> Result<Action>;
}

#[derive(Clone)]
pub enum GenericStrategy {
  Human(HumanStrategy),
  AI(AIStrategy),
  Random(RandomStrategy),
  MonteCarlo(MonteCarloStrategy),
}

impl Strategy for GenericStrategy {
  async fn generate_dispatch(&mut self, game: &Game) -> Result<Action> {
    match self {
      GenericStrategy::Human(val) => val.generate_dispatch(game).await,
      GenericStrategy::AI(val) => val.generate_dispatch(game).await,
      GenericStrategy::Random(val) => val.generate_dispatch(game).await,
      GenericStrategy::MonteCarlo(val) => {
        val.generate_dispatch(game).await
      }
    }
  }
}
