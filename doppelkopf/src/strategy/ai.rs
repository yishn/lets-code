use crate::{
  core::{Action, Game, PlayerIndex},
  strategy::{GenerateDispatchError, Strategy, describe::describe_game_state},
};
use anyhow::Result;
use rig::{
  Agent,
  client::{AgentClientExt, ProviderClient},
  completion::TypedPrompt,
  memory::InMemoryConversationMemory,
  providers::openai::{self, responses_api::GenericResponsesCompletionModel},
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
pub struct AIStrategy {
  player: PlayerIndex,
  agent: Agent<GenericResponsesCompletionModel>,
}

impl AIStrategy {
  pub fn new(player: PlayerIndex) -> Result<Self> {
    let agent = openai::Client::from_env()?
      .agent("gpt-5.6-terra")
      .preamble("You are a Doppelkopf player.")
      .memory(InMemoryConversationMemory::new())
      .conversation("game_history")
      .build();

    Ok(Self { player, agent })
  }
}

impl Strategy for AIStrategy {
  async fn generate_dispatch(&self, game: &Game) -> Result<Action> {
    if game.has_ended() {
      return Err(GenerateDispatchError::GameEnded.into());
    } else {
      let prompt = describe_game_state(self.player, &game);

      let mut retry = false;

      for _ in 0..3 {
        #[derive(Deserialize, JsonSchema)]
        struct UsizeResponse {
          card_number: usize,
        }

        // Send a prompt and await the model's reply.
        let response: UsizeResponse = self.agent
          .prompt_typed(if !retry {
            prompt.clone()
          } else {
            format!("The card you chose was invalid. Please choose another one.\n\n{}", prompt)
          })
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
