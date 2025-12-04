use deck::full_deck;

mod deck;

fn main() {
    let x = full_deck();
    for i in x {
        println!("{} {}", i.value as i32, i.suit as i32);
    }
    println!("Hello, world!");
}
