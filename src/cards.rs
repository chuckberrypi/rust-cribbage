use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, IntoEnumIterator, PartialEq)]
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

#[derive(Debug)]
pub struct Card {
    pub val: Value,
    pub suit: Suit,
}

impl Suit {
    pub fn to_char(&self) -> char {
        match self {
            Suit::Hearts => '\u{2665}',
            Suit::Spades => '\u{2660}',
            Suit::Diamonds => '\u{2666}',
            Suit::Clubs => '\u{2663}',
        }
    }
}
