use crate::card::{Card, Suit, Value};
use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

pub fn full_deck() -> Vec<Card> {
    let values = [
        Value::Nine,
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ten,
        Value::Ace,
    ];
    let suits = [Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];

    let mut deck = Vec::with_capacity(24);
    for &s in &suits {
        for &v in &values {
            deck.push(Card { value: v, suit: s });
        }
    }
    deck
}

pub fn shuffle_cards(cards: &mut Vec<Card>) {
    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    let _ = irs.shuffle(cards, &mut rng);
}