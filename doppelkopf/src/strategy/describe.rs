use crate::core::{Game, PlayerIndex};

fn describe_players(player: PlayerIndex, _game: &Game) -> String {
  format!("There are four players. You are player {}.", player)
}

fn describe_trumps(_player: PlayerIndex, game: &Game) -> String {
  format!(
    "In the current game the following cards are trumps, ordered from highest to lowest:\n{}",
    game
      .trumps()
      .iter()
      .map(|card| card.to_string())
      .reduce(|mut acc, x| {
        acc += " ";
        acc += &x;
        acc
      })
      .unwrap_or("(No trumps)".into())
  )
}

fn describe_last_trick(_player: PlayerIndex, game: &Game) -> String {
  match (game.last_winner(), game.last_trick()) {
    (Some(last_winner), Some(last_trick)) => format!(
      "Last trick was won by player {}. He won the following trick:\n{}",
      last_winner,
      last_trick
        .iter()
        .enumerate()
        .map(|(i, (player, card))| {
          format!("{: >2}. Player {} played {}", i + 1, player, card)
        })
        .reduce(|mut acc, x| {
          acc += "\n";
          acc += &x;
          acc
        })
        .unwrap_or("(No cards yet)".into())
    ),
    _ => format!(""),
  }
}

fn describe_wedding(_player: PlayerIndex, game: &Game) -> String {
  if let Some(declarer) = game.wedding_declarer() {
    format!(
      "A wedding is underway. Player {} is looking for a partner.",
      declarer
    )
  } else {
    "".into()
  }
}

fn describe_current_trick(_player: PlayerIndex, game: &Game) -> String {
  format!(
    "The current trick consists of the following cards so far:\n{}",
    game
      .current_trick()
      .iter()
      .enumerate()
      .map(|(i, (player, card))| {
        format!("{: >2}. Player {} played {}", i + 1, player, card)
      })
      .reduce(|mut acc, x| {
        acc += "\n";
        acc += &x;
        acc
      })
      .unwrap_or("(No cards yet)".into())
  )
}

fn describe_player(player: PlayerIndex, game: &Game) -> String {
  let player = game.player(player);

  if let Some(player) = player {
    format!(
      "It’s your turn. You are team {}. Choose one of the cards on your hand to play.\nRespond with the number of the corresponding card:\n{}",
      player.team(),
      player
        .cards()
        .iter()
        .enumerate()
        .map(|(i, card)| format!("{: >2}. {}", i + 1, card))
        .reduce(|mut acc, x| {
          acc += "\n";
          acc += &x;
          acc
        })
        .unwrap_or("(No cards)".into())
    )
  } else {
    format!("(Current player not found)")
  }
}

pub fn describe_game_state(player: PlayerIndex, game: &Game) -> String {
  if game.game_winners().len() > 0 {
    format!(
      "The game has ended. Players {} have won with {} points.",
      game
        .game_winners()
        .iter()
        .map(|player| player.to_string())
        .reduce(|mut acc, x| {
          acc += " ";
          acc += &x;
          acc
        })
        .unwrap(),
      game
        .game_winners()
        .iter()
        .map(|&player| game.player(player).unwrap().points())
        .sum::<isize>()
    )
  } else {
    [
      describe_players(player, game),
      describe_trumps(player, game),
      describe_last_trick(player, game),
      describe_wedding(player, game),
      describe_current_trick(player, game),
      describe_player(player, game),
    ]
    .into_iter()
    .filter(|msg| !msg.is_empty())
    .reduce(|mut acc, x| {
      acc += "\n\n";
      acc += &x;
      acc
    })
    .unwrap_or_default()
  }
}
