use deck::full_deck;

mod deck;

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

fn main() {
    let mut cards = full_deck();
    
    for i in &cards {
        println!("{} {}", i.value as i32, i.suit as i32);
    }

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();

    irs.shuffle(&mut cards, &mut rng);

    for i in cards {
        println!("{} {}", i.value as i32, i.suit as i32);
    }
}