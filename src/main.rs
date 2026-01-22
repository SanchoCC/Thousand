use crate::card::Card;
use crate::deck::shuffle_cards;
use crate::player::Player;
use deck::full_deck;
use std::io::{self, Write};

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
    let mut last_suit = cards[0].suit;
    for card in cards {
        if last_suit != card.suit {
            print!("  ");
            last_suit = card.suit;
        }
        print!("{}{} ", card.value, card.suit);
    }
}

fn main() {
    let num_players = 3;
    let mut players: Vec<Player> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        players.push(Player::default());
    }

    let mut start_index = 0;
    loop {
        let mut cards = full_deck();
        shuffle_cards(&mut cards);
        let mut stock_cards: Vec<Card> = Vec::new();
        deal_cards(&mut cards, &mut players, &mut stock_cards, start_index);
        let mut bet = 100;

        for player in &mut players {
            println!("Current player: {}, {} points.", player.name, player.score);
            println!("Enter command: 1: Print, 2: Sort Asc, 3: Sort Desc, 4: Raise to: {}, 5: Fold, Other: Quit", bet+10);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    print_cards(&player.hand);
                    println!();
                }
                "2" => {
                    // Ascending sort
                    player.hand.sort();
                    println!("Cards sorted (Ascending).");
                }
                "3" => {
                    // Descending sort
                    player.hand.sort_by(|a, b| b.cmp(a));
                    println!("Cards sorted (Descending).");
                }
                "4" => {
                    // Raise
                }
                "5" => {
                    // Fold
                }
                _ => continue,
            }
        }

        for player in &mut players {
            player.hand.clear();
        }
        start_index += 1;
        start_index %= players.len();
    }
}
