use crate::card::{Card, Suit, Value};

pub struct Rules;

impl Rules {
    #[inline]
    pub fn get_card_points(card: &Card) -> i32 {
        card.value.points()
    }

    #[inline]
    pub fn get_marriage_points(suit: Suit) -> i32 {
        suit.marriage_points()
    }

    pub fn is_marriage(hand: &[Card], card: &Card) -> bool {
        let partner_value = match card.value {
            Value::King => Value::Queen,
            Value::Queen => Value::King,
            _ => return false,
        };
        hand.iter()
            .any(|c| c.suit == card.suit && c.value == partner_value)
    }

    pub fn validate_move(
        hand: &[Card],
        card: &Card,
        lead_suit: Option<Suit>,
        trump_suit: Option<Suit>,
    ) -> bool {
        match lead_suit {
            Some(suit) => {
                if card.suit == suit {
                    return true;
                }

                let has_suit = hand.iter().any(|c| c.suit == suit);
                if has_suit {
                    return false;
                }

                if let Some(trump) = trump_suit {
                    let has_trump = hand.iter().any(|c| c.suit == trump);
                    if has_trump {
                        return card.suit == trump;
                    }
                }

                true
            }
            None => true,
        }
    }

    pub fn determine_trick_winner(
        played: &[(usize, Card)],
        trump_suit: Option<Suit>,
    ) -> (usize, Card) {
        let lead_suit = played[0].1.suit;
        let mut best = played[0];

        for &challenger in &played[1..] {
            let win = best.1;
            let chal = challenger.1;

            if chal.suit == lead_suit {
                if win.suit == lead_suit && chal.value > win.value {
                    best = challenger;
                }
            } else if let Some(trump) = trump_suit {
                if chal.suit == trump {
                    if win.suit != trump || chal.value > win.value {
                        best = challenger;
                    }
                }
            }
        }

        best
    }
}
