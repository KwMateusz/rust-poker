use enum_iterator::IntoEnumIterator;
use std::fmt;

// trait Dedup {
//     fn clear_duplicates(&mut self);
// }

// impl Dedup for Vec<Figure> {
//     fn clear_duplicates(&mut self) {
//         println!("siema");
//         // let mut already_seen = vec![];
//         // self.retain(|item| match already_seen.contains(item) {
//         //     true => false,
//         //     _ => {
//         //         already_seen.push(item.clone());
//         //         true
//         //     }
//         // })
//     }
// }

#[derive(Debug, Copy, Clone, IntoEnumIterator, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs = 2,
    Diamonds = 3,
    Hearts = 4,
    Spades = 1,
}

#[derive(Debug, Copy, Clone, IntoEnumIterator, Eq, Ord, PartialEq, PartialOrd)]
pub enum Figure {
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub figure: Figure,
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Figure::Ace => write!(f, "A"),
            Figure::Two => write!(f, "2"),
            Figure::Three => write!(f, "3"),
            Figure::Four => write!(f, "4"),
            Figure::Five => write!(f, "5"),
            Figure::Six => write!(f, "6"),
            Figure::Seven => write!(f, "7"),
            Figure::Eight => write!(f, "8"),
            Figure::Nine => write!(f, "9"),
            Figure::Ten => write!(f, "10"),
            Figure::Jack => write!(f, "J"),
            Figure::Queen => write!(f, "Q"),
            Figure::King => write!(f, "K"),
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.figure, self.suit)
    }
}