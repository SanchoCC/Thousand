#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Nine = 0,
    Jack = 2,
    Queen = 3,
    King = 4,
    Ten = 10,
    Ace = 11,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spade = 40,
    Club = 60,
    Diamonds = 80,
    Heart = 100,
}

pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

pub fn full_deck() -> Vec<Card> {
    let values = [
        Value::Nine,
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ten,
        Value::Ace,
    ];
    let suits = [Suit::Spade, Suit::Club, Suit::Diamonds, Suit::Heart];

    let mut deck = Vec::with_capacity(24);
    for &s in &suits {
        for &v in &values {
            deck.push(Card { value: v, suit: s });
        }
    }
    deck
}
