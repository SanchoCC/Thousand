use crate::card::Card;
use crate::card::Suit;
use crate::deck::shuffle_cards;
use crate::player::Player;
use crate::rules::Rules;
use deck::full_deck;
use std::io::{self, Write};

mod card;
mod deck;
mod player;
mod rules;

fn deal_cards(
    cards: &mut Vec<Card>,
    players: &mut Vec<Player>,
    stock_cards: &mut Vec<Card>,
    start_index: usize,
) {
    if players.len() >= 3 {
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

fn print_cards(cards: &[Card], show_index: bool) {
    let mut last_suit = cards[0].suit;
    if show_index {
        print!("0: ")
    }
    for i in 0..cards.len() {
        let card = cards[i];
        if last_suit != card.suit {
            print!("  ");
            if show_index {
                print!("{}: ", i)
            }
            last_suit = card.suit;
        }
        print!("{}{} ", card.value, card.suit);
    }
    println!();
}

fn setup_players(num_players: usize) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        players.push(Player::default());
        players[i].name = i.to_string();
    }
    players
}

fn run_bidding(
    players: &mut Vec<Player>,
    start_index: usize,
    num_players: usize,
) -> (usize, usize) {
    let mut bet = 100usize;
    let mut bet_player = start_index;
    let mut pass_count = 0;
    let mut cur_player = start_index + 1;
    let mut player_pass: Vec<bool> = vec![false, false, false];
    loop {
        println!();
        if pass_count == 2 {
            break;
        }
        let player_index = cur_player % num_players;
        if player_pass[player_index] {
            cur_player += 1;
            continue;
        }
        let bet_player_name = players[bet_player].name.clone();
        let player = &mut players[player_index];
        println!(
            "Current player: {}, {} points. Bet player: {} with bet: {}.",
            player.name, player.score, bet_player_name, bet
        );
        println!(
            "Enter command: 1: Print, 2: Sort Asc, 3: Sort Desc, 4: Raise to: {}, 5: Pass, Other: Quit",
            bet + 10
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print_cards(&player.hand, false);
            }
            "2" => {
                player.hand.sort();
                println!("Cards sorted (Ascending).");
            }
            "3" => {
                player.hand.sort_by(|a, b| b.cmp(a));
                println!("Cards sorted (Descending).");
            }
            "4" => {
                bet_player = player_index;
                bet += 10;
                cur_player += 1;
            }
            "5" => {
                pass_count += 1;
                player_pass[player_index] = true;
                cur_player += 1;
            }
            _ => continue,
        }
    }

    (bet_player, bet)
}

fn exchange_cards_from_winner(
    players: &mut Vec<Player>,
    bet_player: usize,
    bet: usize,
    num_players: usize,
) {
    let bet_player_name = players[bet_player].name.clone();
    let bet_player_score = players[bet_player].score.clone();
    let mut given_cards = 0;
    loop {
        if given_cards == 2 {
            break;
        }

        println!(
            "Current player: {}, {} points. Bet: {}.",
            bet_player_name, bet_player_score, bet
        );
        println!("Enter command: 1: Print, 2: Sort Asc, 3: Sort Desc, 4: Give card, Other: Quit");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print_cards(&players[bet_player].hand, true);
            }
            "2" => {
                players[bet_player].hand.sort();
                println!("Cards sorted (Ascending).");
            }
            "3" => {
                players[bet_player].hand.sort_by(|a, b| b.cmp(a));
                println!("Cards sorted (Descending).");
            }
            "4" => {
                print_cards(&players[bet_player].hand, true);
                println!("Select index of card to give!");
                let mut line = String::new();
                let give_card_index = loop {
                    line.clear();
                    io::stdin().read_line(&mut line).unwrap();
                    match line.trim().parse::<usize>() {
                        Ok(n) => break n,
                        Err(_) => println!("Incorrect input, type number"),
                    }
                };

                if give_card_index >= players[bet_player].hand.len() {
                    println!("Incorrect index, out of range");
                    continue;
                }

                let card = players[bet_player].hand.remove(give_card_index);

                given_cards += 1;

                players[(bet_player + given_cards) % num_players]
                    .hand
                    .push(card);
            }
            _ => continue,
        }
    }
}

fn choose_final_bet(initial_bet: usize) -> usize {
    loop {
        println!(
            "Choose final bet of the game. Minimum bet is {}.",
            initial_bet
        );
        let mut line = String::new();
        let final_bet = loop {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            match line.trim().parse::<usize>() {
                Ok(n) => break n,
                Err(_) => println!("Incorrect input, type number"),
            }
        };

        if final_bet < initial_bet || final_bet > 300 {
            continue;
        }
        return final_bet;
    }
}

fn run_play(players: &mut Vec<Player>, declarer: usize, bet: usize, num_players: usize) {
    let mut leader = declarer;
    let mut trump_suit: Option<Suit> = None;
    let mut round_points = vec![0usize; num_players];

    while players[0].hand.len() > 0 {
        if let Some(t) = trump_suit {
            println!("Current Trump: {}", t);
        } else {
            println!("No Trump set.");
        }
        println!("Starting trick. Leader: {}", players[leader].name);

        let mut played: Vec<(usize, Card)> = Vec::with_capacity(num_players);
        let mut lead_suit: Option<Suit> = None;

        for i in 0..num_players {
            let player_index = (leader + i) % num_players;
            loop {
                println!("{}'s turn.", players[player_index].name);
                print_cards(&players[player_index].hand, true);
                println!("Select index of card to play:");
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                match line.trim().parse::<usize>() {
                    Ok(idx) if idx < players[player_index].hand.len() => {
                        let card_to_play = players[player_index].hand[idx];

                        if !Rules::validate_move(
                            &players[player_index].hand,
                            &card_to_play,
                            lead_suit,
                            trump_suit,
                        ) {
                            println!("Invalid move! You must follow suit or trump if applicable.");
                            continue;
                        }

                        if i == 0 && players[player_index].has_taken_trick {
                            if Rules::is_marriage(&players[player_index].hand, &card_to_play) {
                                trump_suit = Some(card_to_play.suit);
                                let points = Rules::get_marriage_points(card_to_play.suit);
                                round_points[player_index] += points as usize;
                                println!(
                                    "Marriage declared! {} points added. Trump is now {}",
                                    points, card_to_play.suit
                                );
                            }
                        }

                        let card = players[player_index].hand.remove(idx);
                        if i == 0 {
                            lead_suit = Some(card.suit);
                        }

                        println!(
                            "{} played {}{}",
                            players[player_index].name, card.value, card.suit
                        );
                        played.push((player_index, card));
                        break;
                    }
                    _ => println!("Incorrect input, type number within range"),
                }
            }
        }

        let winning = Rules::determine_trick_winner(&played, trump_suit);

        let mut trick_points = 0;
        for &(_, card) in &played {
            trick_points += Rules::get_card_points(&card);
        }

        round_points[winning.0] += trick_points as usize;
        players[winning.0].has_taken_trick = true;

        println!(
            "{} wins the trick with {}{}. Points: {}",
            players[winning.0].name, winning.1.value, winning.1.suit, trick_points
        );
        leader = winning.0;
    }

    println!("Round finished. Round Points:");
    for i in 0..num_players {
        println!("{}: {}", players[i].name, round_points[i]);
    }

    for i in 0..num_players {
        if i == declarer {
            if round_points[i] >= bet {
                players[i].score += bet as i32;
            } else {
                players[i].score -= bet as i32;
            }
        } else {
            let rounded = ((round_points[i] + 5) / 10) * 10;
            players[i].score += rounded as i32;
        }
    }

    println!("Total Game Scores:");
    for p in players.iter() {
        println!("{}: {}", p.name, p.score);
    }
}

fn sort_hands(players: &mut Vec<Player>) {
    for player in &mut *players {
        player.hand.sort();
    }
}

fn clear_hands(players: &mut Vec<Player>) {
    for player in &mut *players {
        player.hand.clear();
        player.has_taken_trick = false;
    }
}

fn play_round(players: &mut Vec<Player>, start_index: usize, num_players: usize) -> usize {
    let mut cards = full_deck();
    shuffle_cards(&mut cards);
    let mut stock_cards: Vec<Card> = Vec::new();
    deal_cards(&mut cards, players, &mut stock_cards, start_index);

    sort_hands(players);

    let (bet_player, mut bet) = run_bidding(players, start_index, num_players);

    let bet_player_name = players[bet_player].name.clone();
    println!("Player {} won the trade with {} bet!", bet_player_name, bet);
    if bet > 100 {
        println!("Cards in stock:");
        print_cards(&stock_cards, false);
    }

    players[bet_player].hand.append(&mut stock_cards);
    players[bet_player].hand.sort();

    exchange_cards_from_winner(players, bet_player, bet, num_players);

    sort_hands(players);

    print_cards(&players[bet_player].hand, true);

    bet = choose_final_bet(bet);

    players[bet_player].has_taken_trick = true;

    run_play(players, bet_player, bet, num_players);

    clear_hands(players);

    let mut next_start = start_index + 1;
    next_start %= players.len();
    next_start
}

fn main() {
    let num_players = 3;
    let mut players = setup_players(num_players);
    let mut start_index = 0;
    loop {
        start_index = play_round(&mut players, start_index, num_players);
    }
}
