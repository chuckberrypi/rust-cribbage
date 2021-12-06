use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, IntoEnumIterator)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Clone, Copy, IntoEnumIterator)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub val: Value,
    pub suit: Suit,
}

// impl fmt::Debug for Card {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Ace => "A",
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Nine => "9",
                Value::Ten => "T",
                Value::Jack => "J",
                Value::Queen => "Q",
                Value::King => "K",
            }
        )
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Hearts => '\u{2665}',
                Suit::Spades => '\u{2660}',
                Suit::Diamonds => '\u{2666}',
                Suit::Clubs => '\u{2663}',
            }
        )
    }
}

pub struct Deck(pub Vec<Card>);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.val, self.suit)
    }
}

fn rand_swap_remove<T>(v: &mut Vec<T>, r: &mut rand::rngs::ThreadRng) -> Option<T> {
    if v.len() == 0 {
        return None;
    }
    let i = r.gen_range(0..v.len());
    Some(v.swap_remove(i))
}

// impl From<&Vec<&Card>> for Deck {
//     fn from(v: [Card]) -> Self {
//         let cards: Vec<Card> = Vec::new();
//         Deck(cards.clone_from_slice(v))
//     }
// }

impl From<&[Card]> for Deck {
    fn from(c: &[Card]) -> Self {
        let mut v: Vec<Card> = Vec::new();
        v.clone_from_slice(c);
        Deck(v)
    }
}

impl From<&Vec<&Card>> for Deck {
    fn from(v: &Vec<&Card>) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for card in v.iter() {
            cards.push((**card).clone());
        }
        Deck(cards)
    }
}

impl Deck {
    // pub fn from(c: &Vec<&Card>) -> Self {
    // pub fn from(c: &[&Card]) -> Self {
    //     let mut cards: Vec<Card> = Vec::new();
    //     for card in c.iter() {
    //         cards.push((**card).clone());
    //     }
    //     Deck(cards)
    // }

    pub fn new() -> Self {
        let mut d: Vec<Card> = Vec::with_capacity(52);
        for s in Suit::into_enum_iter() {
            for v in Value::into_enum_iter() {
                d.push(Card { val: v, suit: s });
            }
        }
        Deck(d)
    }

    pub fn new_shuffled() -> Self {
        let mut rng = thread_rng();
        let mut d: Vec<Card> = Vec::with_capacity(52);
        for _ in 0..52 {
            d.push(Card {
                val: Value::Ace,
                suit: Suit::Hearts,
            });
        }
        let mut positions: Vec<usize> = Vec::with_capacity(52);
        for i in 0..52 as usize {
            positions.push(i);
        }
        // println!("Positions: {:?}", positions);
        for s in Suit::into_enum_iter() {
            for v in Value::into_enum_iter() {
                if let Some(i) = rand_swap_remove(&mut positions, &mut rng) {
                    d[i] = Card { suit: s, val: v };
                }
            }
        }
        Deck(d)
    }

    pub fn shuffle(self) -> Self {
        let mut rng = thread_rng();
        let mut positions: Vec<usize> = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            positions.push(i);
        }
        let mut d: Vec<Card> = Vec::with_capacity(self.0.len());
        while let Some(i) = rand_swap_remove(&mut positions, &mut rng) {
            d.push(self.0[i].clone());
        }
        Self(d)
    }

    pub fn swap_remove_rand(&mut self, r: &mut rand::rngs::ThreadRng) -> Card {
        self.0.swap_remove(r.gen_range(0..self.0.len()))
    }

    pub fn deal_rand_to(&mut self, recipient: &mut Deck, r: &mut rand::rngs::ThreadRng) {
        recipient.0.push(self.swap_remove_rand(r));
    }

    pub fn combos(&self, n: usize) -> Vec<Deck> {
        self.0
            .iter()
            .combinations(n)
            .map(|c| Deck::from(&c))
            .collect()
    }
}
