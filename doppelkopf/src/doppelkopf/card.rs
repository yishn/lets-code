use std::{cmp::Ordering, ops::Deref};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
  Nine = 0,
  Jack = 2,
  Queen = 3,
  King = 4,
  Ten = 10,
  Ace = 11,
}

impl Rank {
  pub fn iter() -> impl Iterator<Item = Rank> {
    vec![
      Rank::Nine,
      Rank::Jack,
      Rank::Queen,
      Rank::King,
      Rank::Ten,
      Rank::Ace,
    ]
    .into_iter()
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
  Club,
  Spade,
  Heart,
  Diamond,
}

impl Suit {
  pub fn iter() -> impl Iterator<Item = Suit> {
    vec![Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond].into_iter()
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card {
  pub rank: Rank,
  pub suit: Suit,
}

impl Card {
  pub fn iter() -> impl Iterator<Item = Card> {
    Suit::iter()
      .flat_map(|suit| Rank::iter().map(move |rank| Card::new(suit, rank)))
  }

  pub fn new(suit: Suit, rank: Rank) -> Self {
    Self { suit, rank }
  }

  pub fn points(&self) -> isize {
    self.rank as isize
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OrderedCard<'a> {
  card: Card,
  trumps: &'a [Card], // From higher trumps to lower trumps
}

impl<'a> OrderedCard<'a> {
  pub fn new(card: Card, trumps: &'a [Card]) -> Self {
    Self { card, trumps }
  }

  pub fn is_trump(&self) -> bool {
    self.trumps.contains(&self.card)
  }
}

impl<'a> Deref for OrderedCard<'a> {
  type Target = Card;

  fn deref(&self) -> &Self::Target {
    &self.card
  }
}

impl<'a> From<OrderedCard<'a>> for Card {
  fn from(value: OrderedCard<'a>) -> Self {
    value.card
  }
}

impl<'a> PartialOrd for OrderedCard<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.trumps != other.trumps {
      return None;
    } else if self.card == other.card {
      return Some(Ordering::Equal);
    }

    match (
      self.trumps.iter().position(|x| x == &self.card),
      self.trumps.iter().position(|x| x == &other.card),
    ) {
      (Some(_), None) => Some(Ordering::Greater),
      (None, Some(_)) => Some(Ordering::Less),
      (Some(i), Some(j)) => Some(if i < j {
        Ordering::Greater
      } else {
        Ordering::Less
      }),
      (None, None) => {
        if self.card.suit != other.card.suit {
          None
        } else {
          self.card.rank.partial_cmp(&other.card.rank)
        }
      }
    }
  }
}
