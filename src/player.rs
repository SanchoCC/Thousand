use crate::card::Card;

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub score: i32,
    pub has_taken_trick: bool,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from("Default"),
            hand: Vec::with_capacity(10),
            score: 0,
            has_taken_trick: false,
        }
    }
}
