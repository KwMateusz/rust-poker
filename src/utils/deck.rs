use crate::utils::card::Card;
use crate::utils::card::Suit;
use crate::utils::card::Figure;

use enum_iterator::IntoEnumIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::into_enum_iter() {
            for figure in Figure::into_enum_iter() {
                if (figure != Figure::None){
                    let card = Card {
                        suit: *(&suit),
                        figure: *(&figure),
                    };
                    cards.push(card);
                }
            }
        }

        Deck { cards }
    }

    pub fn clean(&mut self) {
        self.cards = vec![];
    }

    pub fn take_card(&mut self) -> Card {
        if let Some(c) = self.cards.pop() {
            c
        } else {
            panic!("Could not take card from deck!");
        }
    }
}