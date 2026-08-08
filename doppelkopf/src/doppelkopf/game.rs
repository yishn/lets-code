use super::card::Card;
use crate::doppelkopf::card::{OrderedCard, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Team {
  Re,
  Contra,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerIndex(usize);

#[derive(Debug, Clone)]
pub struct Player {
  team: Team,
  cards: Vec<Card>,
  won_tricks: Vec<Vec<(PlayerIndex, Card)>>,
}

impl Player {
  pub fn new(cards: Vec<Card>) -> Self {
    Self {
      team: if cards
        .iter()
        .any(|&c| c == Card::new(Suit::Club, Rank::Queen))
      {
        Team::Re
      } else {
        Team::Contra
      },
      cards,
      won_tricks: vec![],
    }
  }
}

pub enum Action {
  PlayCard { player: PlayerIndex, card: Card },
  EndGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidDispatch {
  InvalidPlayer,
  InvalidCard,
  InvalidEnd,
  GameEnded,
}

#[derive(Debug, Clone)]
pub struct Game {
  players: Vec<Player>,
  trumps: Vec<Card>,
  wedding: Option<PlayerIndex>,
  trick: Vec<(PlayerIndex, Card)>,
  last_winner: Option<PlayerIndex>,
  game_winners: Vec<PlayerIndex>,
}

impl Game {
  pub fn new() -> Self {
    let mut cards = Card::iter().chain(Card::iter()).collect::<Vec<_>>();
    cards.shuffle(&mut rand::rng());

    let hand_count = cards.len() / 4;
    let players = (0..4)
      .map(|i| {
        Player::new(
          cards[i * hand_count..(i + 1) * hand_count]
            .iter()
            .copied()
            .collect(),
        )
      })
      .collect::<Vec<_>>();

    Self {
      trumps: vec![Card::new(Suit::Heart, Rank::Ten)]
        .into_iter()
        .chain(Suit::iter().map(|suit| Card::new(suit, Rank::Queen)))
        .chain(Suit::iter().map(|suit| Card::new(suit, Rank::Jack)))
        .chain(vec![
          Card::new(Suit::Diamond, Rank::Ace),
          Card::new(Suit::Diamond, Rank::Ten),
          Card::new(Suit::Diamond, Rank::King),
          Card::new(Suit::Diamond, Rank::Nine),
        ])
        .collect(),
      wedding: players
        .iter()
        .position(|player| {
          player
            .cards
            .iter()
            .filter(|&&card| card == Card::new(Suit::Club, Rank::Queen))
            .count()
            >= 2
        })
        .map(|i| PlayerIndex(i)),
      players,
      trick: vec![],
      last_winner: None,
      game_winners: vec![],
    }
  }

  pub fn can_dispatch(&self, action: &Action) -> Option<InvalidDispatch> {
    match action {
      Action::PlayCard { player, card } => {
        // Check game end

        if self.game_winners.len() > 0 {
          return Some(InvalidDispatch::GameEnded);
        }

        // 1. Check player turn

        let current_player = self
          .trick
          .last()
          .map(|last| PlayerIndex((last.0.0 + 1) % 4))
          .unwrap_or_else(|| self.last_winner.unwrap_or(PlayerIndex(0)));

        if player != &current_player {
          return Some(InvalidDispatch::InvalidPlayer);
        }

        // 2. Card existence

        let cards = &self.players[player.0].cards;

        if !cards.contains(card) {
          return Some(InvalidDispatch::InvalidCard);
        }

        // 3. Card should follow suit

        if let Some(&(_, lead_card)) = self.trick.first() {
          let lead_card = OrderedCard::new(lead_card, &self.trumps);
          let card = OrderedCard::new(*card, &self.trumps);

          if lead_card.is_trump()
            && !card.is_trump()
            && cards
              .iter()
              .any(|card| OrderedCard::new(*card, &self.trumps).is_trump())
          {
            return Some(InvalidDispatch::InvalidCard);
          } else if !lead_card.is_trump()
            && (card.is_trump() || lead_card.suit != card.suit)
            && cards.iter().any(|card| {
              !OrderedCard::new(*card, &self.trumps).is_trump()
                && card.suit == lead_card.suit
            })
          {
            return Some(InvalidDispatch::InvalidCard);
          }
        }
      }
      Action::EndGame => {
        if self.players.iter().any(|player| player.cards.len() > 0) {
          return Some(InvalidDispatch::InvalidEnd);
        }
      }
    }

    None
  }

  pub fn dispatch(&mut self, action: Action) -> Result<(), InvalidDispatch> {
    if let Some(err) = self.can_dispatch(&action) {
      return Err(err);
    }

    match action {
      Action::PlayCard { player, card } => {
        // Add card to trick

        self.trick.push((player, card));

        // Remove card from player's hand

        let index = self.players[player.0]
          .cards
          .iter()
          .position(|&c| c == card)
          .unwrap();
        self.players[player.0].cards.remove(index);

        // Handle won trick if applicable

        if self.trick.len() == self.players.len() {
          // Determine winner

          let winner = self
            .trick
            .iter()
            .map(|(player, card)| {
              (*player, OrderedCard::new(*card, &self.trumps))
            })
            .reduce(|(player1, card1), (player2, card2)| {
              if card2 > card1 {
                (player2, card2)
              } else {
                (player1, card1)
              }
            })
            .unwrap()
            .0;

          self.last_winner = Some(winner);

          // Move trick to winner

          let trick = std::mem::take(&mut self.trick);
          self.players[winner.0].won_tricks.push(trick);

          // Handle wedding

          if let Some(declarer) = self.wedding {
            if winner != declarer {
              self.players[winner.0].team = Team::Re;
            }
          }
        }
      }
      Action::EndGame => {
        let mut points = [Team::Re, Team::Contra].into_iter().map(|team| {
          self
            .players
            .iter()
            .filter(move |player| player.team == team)
            .map(|player| {
              player
                .won_tricks
                .iter()
                .flat_map(|x| x.iter().map(|(_, card)| card.points()))
                .sum::<isize>()
            })
            .sum::<isize>()
        });

        let re_points = points.next().unwrap();
        let winning_team = if re_points > 120 {
          Team::Re
        } else {
          Team::Contra
        };

        self.game_winners = self
          .players
          .iter()
          .enumerate()
          .filter(move |(_, player)| player.team == winning_team)
          .map(|(i, _)| PlayerIndex(i))
          .collect();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::doppelkopf::game::Game;

  #[test]
  fn test() {
    let game = Game::new();

    println!("{:#?}", game);
  }
}
