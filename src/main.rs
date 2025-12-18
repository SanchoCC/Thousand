use crate::deck::shuffle_cards;
use deck::full_deck;
use crate::card::Card;
use crate::player::Player;

mod card;
mod deck;
mod player;

fn deal_cards(cards: &mut Vec<Card>, players: &mut Vec<Player>, stock_cards: &mut Vec<Card>) {
    if players.len() >= 3 {
        // Deal cards for first 3 players
        for i in 0..cards.len() {
            let card = cards.pop().unwrap();
        }
    }
}

fn main() {
    let num_players = 3;
    let mut players: Vec<Player> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        players.push(Player::default());
    }
    let mut cards = full_deck();
    shuffle_cards(&mut cards);
}
