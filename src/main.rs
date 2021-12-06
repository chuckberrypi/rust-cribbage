mod cards;

use cards::{Card, Deck};

fn main() {
    let d = Deck::new();
    let d = d.shuffle();
    // for c in d.0.iter() {
    //     println!("{}", c);
    // }

    // println!("===================================");

    let mut d2 = Deck::new_shuffled();
    // for c in d2.0.iter() {
    //     println!("{}", c);
    // }

    let mut rng = rand::thread_rng();
    println!("Hello, world! {}", d2.swap_remove_rand(&mut rng));
    // let cards: Vec<&Card> = d.0.iter().collect();
    // let d2 = Deck::from(&&d.0[0..4]);
    let d2 = Deck::from(&d.0[0..4]);
    // for c in d2.combos(3).iter() {
    //     println!("{:?}", c.0);
    // }
    // println!("{:?}", d2.0);
}
