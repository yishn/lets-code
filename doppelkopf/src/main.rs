#![allow(irrefutable_let_patterns)]
#![allow(unused)]

use self::{
  core::Team,
  strategy::{GenericStrategy, MonteCarloStrategy, RandomStrategy},
};
use crate::{
  core::{Action, Game, PlayerIndex},
  strategy::{AIStrategy, HumanStrategy, Strategy, describe_game_state},
};
use anyhow::Result;

mod core;
mod strategy;

async fn human_vs_ai() -> Result<()> {
  let human_stategy = HumanStrategy::new(PlayerIndex(0));
  let ai_strategies = (1..4).map(|i| AIStrategy::new(PlayerIndex(i)).unwrap());
  let mut strategies = [GenericStrategy::Human(human_stategy)]
    .into_iter()
    .chain(ai_strategies.into_iter().map(GenericStrategy::AI))
    .collect::<Vec<_>>();

  let mut game = Game::new();

  while !game.has_ended() {
    for i in 0..4 {
      let current_player =
        PlayerIndex((game.last_winner().unwrap_or_default().0 + i) % 4);
      let strategy = &mut strategies[current_player.0];
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

async fn human_vs_monte_carlo() -> Result<()> {
  let human_stategy = HumanStrategy::new(PlayerIndex(0));
  let ai_strategies =
    (1..4).map(|i| MonteCarloStrategy::new(PlayerIndex(i), 25000));
  let mut strategies = [GenericStrategy::Human(human_stategy)]
    .into_iter()
    .chain(ai_strategies.into_iter().map(GenericStrategy::MonteCarlo))
    .collect::<Vec<_>>();

  let mut game = Game::new();

  while !game.has_ended() {
    for i in 0..4 {
      let current_player =
        PlayerIndex((game.last_winner().unwrap_or_default().0 + i) % 4);
      let strategy = &mut strategies[current_player.0];
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

async fn random_vs_monte_carlo() -> Result<()> {
  let mut game = Game::new();
  let mut strategies = (0..4)
    .map(PlayerIndex)
    .map(|player| (player, game.player(player).unwrap()))
    .map(|(id, player)| match player.team() {
      Team::Re => GenericStrategy::Random(RandomStrategy),
      Team::Contra => {
        GenericStrategy::MonteCarlo(MonteCarloStrategy::new(id, 25000))
      }
    })
    .collect::<Vec<_>>();

  while !game.has_ended() {
    for i in 0..4 {
      let current_player =
        PlayerIndex((game.last_winner().unwrap_or_default().0 + i) % 4);
      let strategy = &mut strategies[current_player.0];
      let action = strategy.generate_dispatch(&game).await?;

      println!("{}", describe_game_state(current_player, &game));

      if let Action::PlayCard { player, card } = &action {
        println!("> Player {} played {}.", player, card);
      }

      game.dispatch(action)?;
    }
  }

  println!("{}", describe_game_state(PlayerIndex(0), &game));

  Ok(())
}

async fn ai_vs_monte_carlo() -> Result<()> {
  let mut game = Game::new();
  let mut strategies = (0..4)
    .map(PlayerIndex)
    .map(|player| (player, game.player(player).unwrap()))
    .map(|(id, player)| match player.team() {
      Team::Re => {
        GenericStrategy::MonteCarlo(MonteCarloStrategy::new(id, 50000))
      }
      Team::Contra => GenericStrategy::AI(AIStrategy::new(id).unwrap()),
    })
    .collect::<Vec<_>>();

  while !game.has_ended() {
    for i in 0..4 {
      let current_player =
        PlayerIndex((game.last_winner().unwrap_or_default().0 + i) % 4);
      let strategy = &mut strategies[current_player.0];
      let action = strategy.generate_dispatch(&game).await?;

      println!("{}", describe_game_state(current_player, &game));

      if let Action::PlayCard { player, card } = &action {
        println!("> Player {} played {}.", player, card);
      }

      game.dispatch(action)?;
    }
  }

  println!("{}", describe_game_state(PlayerIndex(0), &game));

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  // human_vs_ai().await?;
  human_vs_monte_carlo().await?;
  // random_vs_monte_carlo().await?;
  // ai_vs_monte_carlo().await?;

  Ok(())
}
