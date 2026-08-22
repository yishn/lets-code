use super::Strategy;
use crate::{
  core::{Action, DispatchError, Game},
  strategy::GenerateDispatchError,
};
use anyhow::Result;
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, Copy, Default)]
pub struct RandomStrategy;

impl RandomStrategy {
  pub async fn play_till_end(&mut self, game: &mut Game) -> Result<()> {
    while !game.has_ended() {
      let action = self.generate_dispatch(&game).await?;
      game.dispatch(action)?;
    }

    Ok(())
  }
}

impl Strategy for RandomStrategy {
  async fn generate_dispatch(&mut self, game: &Game) -> Result<Action> {
    let player = game.current_player();
    let cards = game.player(player).unwrap().cards();

    loop {
      let card = *cards
        .choose(&mut rand::rng())
        .ok_or(GenerateDispatchError::GameEnded)?;
      let action = Action::PlayCard { player, card };

      match game.can_dispatch(&action) {
        Some(DispatchError::InvalidCard) => {} // Continue looping
        None => return Ok(Action::PlayCard { player, card }),
        _ => return Err(GenerateDispatchError::InvalidActionGenerated.into()),
      }
    }
  }
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut game = Game::new();
  let player = crate::core::PlayerIndex(0);

  game.dispatch(Action::PlayCard {
    player,
    card: *game.player(player).unwrap().cards().last().unwrap(),
  })?;

  let mut strategy = RandomStrategy;
  let action = strategy.generate_dispatch(&game).await?;

  println!("{:#?}", action);
  game.dispatch(action)?;

  Ok(())
}
