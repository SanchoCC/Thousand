use crate::card::Card;

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub score: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from("Default"),
            hand: Vec::with_capacity(10),
            score: 0,
        }
    }
}
