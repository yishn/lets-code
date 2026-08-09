use crate::core::Action;
use crate::core::Game;
use crate::core::PlayerIndex;
use crate::strategy::describe::describe_game_state;
use crate::strategy::{GenerateDispatchError, Strategy};
use anyhow::Result;
use rig::client::AgentClientExt;
use rig::client::ProviderClient;
use rig::completion::TypedPrompt;
use rig::memory::InMemoryConversationMemory;
use rig::providers::openai;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AIStrategy {
  player: PlayerIndex,
}

impl AIStrategy {
  pub fn new(player: PlayerIndex) -> Self {
    Self { player }
  }
}

impl Strategy for AIStrategy {
  async fn generate_dispatch(&self, game: &Game) -> Result<Action> {
    if game.has_ended() {
      return Err(GenerateDispatchError::GameEnded.into());
    } else {
      let prompt = describe_game_state(self.player, &game);

      // Build an agent: a model plus a system prompt (the "preamble").
      let agent = openai::Client::from_env()?
        .agent("gpt-5.6-luna")
        .preamble("You are a Doppelkopf player.")
        .memory(InMemoryConversationMemory::new())
        .build();

      let mut retry = false;

      for _ in 0..3 {
        #[derive(Deserialize, JsonSchema)]
        struct UsizeResponse {
          card_number: usize,
        }

        // Send a prompt and await the model's reply.
        let response: UsizeResponse = agent
          .prompt_typed(if !retry {
            prompt.clone()
          } else {
            format!("The card you chose was invalid. Please choose another one.\n\n{}", prompt)
          })
          .conversation(&format!("player-{}", self.player.0))
          .await?;

        let card = game
          .player(self.player)
          .and_then(|player| player.cards().get(response.card_number - 1))
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

        retry = true;
      }

      Err(GenerateDispatchError::InvalidActionGenerated.into())
    }
  }
}

#[test]
fn describe_test() {
  let game = Game::new();
  let strategy = AIStrategy {
    player: PlayerIndex(0),
  };

  println!("{}", describe_game_state(strategy.player, &game));
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut game = Game::new();

  let strategies = (0..4)
    .map(|i| AIStrategy {
      player: PlayerIndex(i),
    })
    .collect::<Vec<_>>();

  for strategy in &strategies {
    let prompt = describe_game_state(strategy.player, &game);

    println!("{}", prompt);

    let action = strategy.generate_dispatch(&game).await?;

    println!("{:#?}", action);

    game.dispatch(action)?;
  }

  println!("{:#?}", game);

  Ok(())
}
