mod cards;

use cards::{Card, Value, Suit};

fn main() {
    let c: Card = Card {
        val: Value::Ace,
        suit: Suit::Hearts,
    };

    println!("Hello, world! {:?}", c);
}
