use crate::{
  core::{Action, Game, PlayerIndex},
  strategy::{AIStrategy, HumanStrategy, Strategy, describe_game_state},
};
use anyhow::Result;

mod core;
mod strategy;

#[tokio::main]
async fn main() -> Result<()> {
  #[derive(Debug, Clone)]
  enum GenericStrategy {
    Human(HumanStrategy),
    AI(AIStrategy),
  }

  impl Strategy for GenericStrategy {
    async fn generate_dispatch(
      &self,
      game: &core::Game,
    ) -> anyhow::Result<core::Action> {
      match self {
        GenericStrategy::Human(val) => val.generate_dispatch(game).await,
        GenericStrategy::AI(val) => val.generate_dispatch(game).await,
      }
    }
  }

  let human_stategy = HumanStrategy::new(PlayerIndex(0));
  let ai_strategies = (1..4).map(|i| AIStrategy::new(PlayerIndex(i)));
  let strategies = vec![GenericStrategy::Human(human_stategy)]
    .into_iter()
    .chain(ai_strategies.into_iter().map(GenericStrategy::AI))
    .collect::<Vec<_>>();

  let mut game = Game::new();

  while !game.has_ended() {
    for i in 0..4 {
      let current_player =
        PlayerIndex((game.last_winner().unwrap_or_default().0 + i) % 4);
      let strategy = &strategies[current_player.0];
      let action = strategy.generate_dispatch(&game).await?;

      if let Action::PlayCard { player, card } = &action {
        println!("> Player {} played {}.", player, card);
      }

      game.dispatch(action)?;
    }
  }

  println!("{}", describe_game_state(PlayerIndex(0), &game));

  Ok(())
}
