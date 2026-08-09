use std::{cmp::Ordering, fmt::Display, ops::Deref};

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

impl Display for Rank {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Rank::Nine => "9",
      Rank::Jack => "J",
      Rank::Queen => "Q",
      Rank::King => "K",
      Rank::Ten => "10",
      Rank::Ace => "A",
    }
    .fmt(f)
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

impl Display for Suit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Suit::Club => "♣",
      Suit::Spade => "♠",
      Suit::Heart => "♥",
      Suit::Diamond => "♦",
    }
    .fmt(f)
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

impl Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.suit, self.rank)
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TotalOrderedCard<'a>(OrderedCard<'a>);

impl<'a> TotalOrderedCard<'a> {
  pub fn new(card: Card, trumps: &'a [Card]) -> Self {
    Self(OrderedCard::new(card, trumps))
  }
}

impl<'a> Deref for TotalOrderedCard<'a> {
  type Target = OrderedCard<'a>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a> PartialOrd for TotalOrderedCard<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl<'a> Ord for TotalOrderedCard<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.partial_cmp(&other.0).unwrap_or_else(|| {
      (self.0.suit, self.0.rank).cmp(&(other.0.suit, other.0.rank))
    })
  }
}
