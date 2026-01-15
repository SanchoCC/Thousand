use crate::card::Card;
use crate::deck::shuffle_cards;
use crate::player::Player;
use deck::full_deck;

mod card;
mod deck;
mod player;

fn deal_cards(
    cards: &mut Vec<Card>,
    players: &mut Vec<Player>,
    stock_cards: &mut Vec<Card>,
    start_index: usize,
) {
    if players.len() >= 3 {
        // Deal cards for first 3 players
        for i in 0..cards.len() {
            let card = cards.pop().unwrap();
            let to_stock = (start_index + i) % 8;
            if to_stock == 0 {
                stock_cards.push(card);
            } else {
                let order = (start_index + i) % 3;
                players[order].hand.push(card);
            }
        }
    }
}

fn print_cards(cards: &[Card]) {
    for card in cards {
        print!("{}{} ", card.value, card.suit);
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
    let mut start_index = 0;
    let mut stock_cards: Vec<Card> = Vec::new();
    deal_cards(&mut cards, &mut players, &mut stock_cards, start_index);
    // &players[0].hand.sort();
    print_cards(&players[0].hand);
}
