use super::Strategy;
use crate::{
  core::{Action, Card, Game, PlayerIndex, Rank, Suit},
  strategy::{GenerateDispatchError, RandomStrategy},
};
use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MonteCarloStrategy {
  playouts: usize,
  player: PlayerIndex,
  re_team: HashSet<PlayerIndex>,
}

impl MonteCarloStrategy {
  pub fn new(player: PlayerIndex, playouts: usize) -> Self {
    Self {
      playouts,
      player,
      re_team: HashSet::new(),
    }
  }

  async fn expected_score(&self, game: &Game) -> Result<f64> {
    let mut sum = 0.0;

    for _ in 0..self.playouts {
      let mut random_game =
        Game::from_game_randomized(game, self.player, &self.re_team)
          .ok_or(GenerateDispatchError::InvalidActionGenerated)?;

      RandomStrategy.play_till_end(&mut random_game).await?;

      let win = random_game.game_winners().contains(&self.player);
      let score = if win { 1.0 } else { -1.0 };

      sum += score;
    }

    Ok(sum / (self.playouts as f64))
  }
}

impl Strategy for MonteCarloStrategy {
  async fn generate_dispatch(&mut self, game: &Game) -> Result<Action> {
    // Track Re team

    let queen_of_clubs = Card::new(Suit::Club, Rank::Queen);

    if game
      .player(self.player)
      .map(|player| {
        player.cards().len() == 12 && player.cards().contains(&queen_of_clubs)
      })
      .unwrap_or_default()
    {
      self.re_team.insert(self.player);
    }

    if let Some((re_team_player, _)) = game
      .last_trick()
      .and_then(|trick| trick.iter().find(|(_, card)| card == &queen_of_clubs))
    {
      self.re_team.insert(*re_team_player);
    }

    if let Some((re_team_player, _)) = game
      .current_trick()
      .iter()
      .find(|(_, card)| card == &queen_of_clubs)
    {
      self.re_team.insert(*re_team_player);
    }

    // Generate action

    let actions = game
      .player(self.player)
      .into_iter()
      .flat_map(|player| player.cards())
      .map(|card| Action::PlayCard {
        player: self.player,
        card: *card,
      })
      .filter(|action| game.can_dispatch(action).is_none())
      .collect::<Vec<_>>();

    if actions.len() == 0 {
      return Err(GenerateDispatchError::GameEnded.into());
    }

    let mut expected_scores = vec![];

    for action in actions {
      let mut game = game.clone();
      game.dispatch(action.clone())?;
      expected_scores.push((action, self.expected_score(&game).await?));
    }

    let (best_action, _) = expected_scores
      .into_iter()
      .reduce(|(best_action, best_score), (action, score)| {
        if score > best_score {
          (action, score)
        } else {
          (best_action, best_score)
        }
      })
      .unwrap();

    Ok(best_action)
  }
}

#[tokio::test]
async fn test() -> Result<()> {
  let game = Game::new();
  let mut strategy = MonteCarloStrategy::new(PlayerIndex(0), 25000);

  let action = strategy.generate_dispatch(&game).await?;
  println!("{:?}", action);

  Ok(())
}
