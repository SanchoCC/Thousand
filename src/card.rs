use std::cmp::Ordering;
use std::fmt;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    #[default]
    Nine = 0,
    Jack = 2,
    Queen = 3,
    King = 4,
    Ten = 10,
    Ace = 11,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nine => write!(f, "9"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
            Value::Ten => write!(f, "10"),
            Value::Ace => write!(f, "A"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    #[default]
    Spade = 40,
    Club = 60,
    Diamond = 80,
    Heart = 100,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}
