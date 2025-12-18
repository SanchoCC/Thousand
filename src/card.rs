#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    #[default]
    Nine = 0,
    Jack = 2,
    Queen = 3,
    King = 4,
    Ten = 10,
    Ace = 11,
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    #[default]
    Spade = 40,
    Club = 60,
    Diamonds = 80,
    Heart = 100,
}
#[derive(Default, Clone, Copy)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}
