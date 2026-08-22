use crate::{
  core::{Action, Game, PlayerIndex},
  strategy::{GenerateDispatchError, Strategy, describe::describe_game_state},
};
use anyhow::Result;
use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct HumanStrategy {
  player: PlayerIndex,
}

impl HumanStrategy {
  pub fn new(player: PlayerIndex) -> Self {
    Self { player }
  }
}

impl Strategy for HumanStrategy {
  async fn generate_dispatch(&mut self, game: &Game) -> Result<Action> {
    if game.has_ended() {
      return Err(GenerateDispatchError::GameEnded.into());
    } else {
      let prompt = describe_game_state(self.player, &game);

      let mut retry = false;

      loop {
        println!(
          "{}",
          if !retry {
            prompt.clone()
          } else {
            format!(
              "The card you chose was invalid. Please choose another one.\n\n{}",
              prompt
            )
          }
        );

        let response = std::io::stdin().lock().lines().next().unwrap()?;
        let response = response.parse::<usize>();

        if let Ok(response) = response {
          let card = game
            .player(self.player)
            .and_then(|player| player.cards().get(response - 1))
            .copied();

          if let Some(card) = card {
            let action = Action::PlayCard {
              player: self.player,
              card,
            };

            if game.can_dispatch(&action).is_none() {
              return Ok(action);
            }
          }
        }

        retry = true;
      }
    }
  }
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut game = Game::new();
  let mut strategy = HumanStrategy {
    player: PlayerIndex(0),
  };

  let action = strategy.generate_dispatch(&game).await?;

  println!("{:#?}", action);

  game.dispatch(action)?;

  println!("{:#?}", game);

  Ok(())
}
