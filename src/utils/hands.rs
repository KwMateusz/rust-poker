use itertools::Itertools;
use std::collections::HashMap;

use crate::utils::card::*;
use crate::utils::player::Player;


#[derive(Debug, Clone, PartialEq)]
pub enum Hands {
    None,
    HighCard(Figure),
    OnePair(Figure),
    TwoPairs(Figure, Figure),
    ThreeOfAKind(Figure),
    Straight(Figure),
    Flush(Suit),
    FullHouse(Figure),
    FourOfAKind(Figure),
    StraightFlush(Card),
    RoyalFlush,
}

#[derive(Debug, Clone)]
pub struct CardComparer {}

impl CardComparer {
    
    pub fn get_better_hands<'a>(players: &'a mut Vec<&'a mut Player>, card1: Card, card2: Card, card3: Card, turn: Card, river: Card ) {
        for player in players {
            let mut cards = vec![];
            if let Some((c1, c2)) = &player.hand {
                cards.push(c1);
                cards.push(c2);
                cards.push(&card1);
                cards.push(&card2);
                cards.push(&card3);
                cards.push(&turn);
                cards.push(&river);
            } 

            let mut hands: Hands = Hands::None;

            hands = CardComparer::check_high_card(&mut cards);
            hands = CardComparer::check_one_pair(&mut cards);
            hands = CardComparer::check_two_pairs(&mut cards);
            hands = CardComparer::check_three_of_a_kind(&mut cards);
            hands = CardComparer::check_straight(&mut cards);
            hands = CardComparer::check_flush(&mut cards);
            hands = CardComparer::check_full_house(&mut cards);
            hands = CardComparer::check_four_of_a_kind(&mut cards);

            for card in cards {
                print!("card: {} \t", card);
            }
            println!("");
            println!("{:?}", hands);
        }
    } 

    pub fn check_four_of_a_kind(cards: &mut Vec<&Card>) -> Hands {
        
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        figures.sort_by(|a,b| b.cmp(&a));

        let mut duplicated_figures: Vec<Figure> = CardComparer::get_duplicated_card(&mut figures);
        let mut duplicated_figures: Vec<Figure> = CardComparer::get_duplicated_card(&mut duplicated_figures);
        let mut duplicated_figures: Vec<Figure> = CardComparer::get_duplicated_card(&mut duplicated_figures);
       
        duplicated_figures.sort_by(|a,b| b.cmp(&a));

        if duplicated_figures.len() > 0 {
            return Hands::FourOfAKind(*&duplicated_figures[0]);
        }
      
        Hands::None
    }

    pub fn check_full_house(cards: &mut Vec<&Card>) -> Hands {
        
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        figures.sort_by(|a,b| b.cmp(&a));

        let mut previous_figure: i8 = 0;
        let mut figure_index: i8 = 0;

        let mut counter: u8 = 1;
        let mut three_cards_figure: Figure = Figure::None;
        let mut two_cards_figure: Figure = Figure::None;

        for (_i, figure) in figures.iter().enumerate() {
            figure_index = *figure as i8;
            if (_i > 0) {
                if (previous_figure == figure_index) {
                    counter += 1;
                    if (counter > 2) {
                        if (three_cards_figure == Figure::None) {
                            three_cards_figure = figures[_i];
                        }
                    }
                    if (counter > 1) {
                        if (two_cards_figure == Figure::None) {
                            two_cards_figure = figures[_i];
                        }
                    }
                }
                else {
                    counter = 1;
                }
            }
           
            previous_figure = figure_index;
        }
        if (three_cards_figure != Figure::None && two_cards_figure != Figure::None) {
            return Hands::FullHouse(three_cards_figure)
        }
        Hands::None
    }

    pub fn check_flush(cards: &mut Vec<&Card>) -> Hands {
        
        let mut suits: Vec<Suit> = cards.iter().map(|x| x.suit).collect();
        suits.sort_by(|a,b| b.cmp(&a));

        let mut suit_previous: i8 = 0;
        let mut suit_actual: i8 = 0;
        let mut counter: u8 = 1;

        for (_i, suit) in suits.iter().enumerate() {
            suit_actual = *suit as i8;
            if (_i > 0) {
                if (suit_actual == suit_previous) {
                    if (counter == 4) {
                        return Hands::Flush(*suit)
                    }
                    counter += 1;
                }
                else {
                    counter = 1;
                }
            }
            suit_previous = suit_actual;
        }
        Hands::None
    }

    pub fn check_straight(cards: &mut Vec<&Card>) -> Hands {
        
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        figures.sort_by(|a,b| b.cmp(&a));

        let mut figure_index_previous: i8 = 0;
        let mut figure_index: i8 = 0;
        let mut counter: u8 = 1;

        for (_i, figure) in figures.iter().enumerate() {
            figure_index = *figure as i8;
            if (_i > 0) {
                if (figure_index_previous - figure_index == 1) {
                    if (counter == 4) {
                        return Hands::Straight(figures[_i - (4 as usize)])
                    }
                    counter += 1;
                }
                else {
                    counter = 1;
                }
            }
            figure_index_previous = figure_index;
        }
        Hands::None
    }

    pub fn check_three_of_a_kind(cards: &mut Vec<&Card>) -> Hands {
        
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        let mut duplicated_figures: Vec<Figure> = CardComparer::get_duplicated_card(&mut figures);
        let mut duplicated_figures: Vec<Figure> = CardComparer::get_duplicated_card(&mut duplicated_figures);

        duplicated_figures.sort_by(|a,b| b.cmp(&a));

        if duplicated_figures.len() > 0 {
            return Hands::ThreeOfAKind(*&duplicated_figures[0]);
        }
        Hands::None
    }

    pub fn check_one_pair(cards: &mut Vec<&Card>) -> Hands {

        let descending_pairs: Vec<Figure> = CardComparer::descending_pairs(cards);

        if descending_pairs.len() > 0 {
            return Hands::OnePair(*&descending_pairs[0])
        }
        Hands::None
    }

    pub fn check_two_pairs(cards: &mut Vec<&Card>) -> Hands {

        let descending_pairs: Vec<Figure> = CardComparer::descending_pairs(cards);

        if descending_pairs.len() > 1 {
            return Hands::TwoPairs(*&descending_pairs[0], *&descending_pairs[1])
        }
        Hands::None
    }

    pub fn check_high_card(cards: &mut Vec<&Card>) -> Hands {
        cards.sort_by(|a,b| b.figure.cmp(&a.figure));
        Hands::HighCard(*&cards[0].figure)
    }

    pub fn get_duplicated_card(figures: &mut Vec<Figure>) -> Vec<Figure> {
        let mut already_seen = vec![];
        figures.retain(|item| match already_seen.contains(item) {
            true => true,
            _ => {
                already_seen.push(item.clone());
                false
            }
        });

        figures.to_vec()
    }

    pub fn remove_duplicated_card(figures: &mut Vec<Figure>) -> Vec<Figure> {
        let mut already_seen = vec![];
        figures.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        });

        figures.to_vec()
    }

    pub fn descending_pairs(cards: &mut Vec<&Card>) -> Vec<Figure> {
        let mut figures: Vec<Figure> = cards.iter().map(|x| x.figure).collect();
        let mut duplicated_cards: Vec<Figure> = CardComparer::get_duplicated_card(&mut figures);
        let mut removed_repeating_duplicates: Vec<Figure> = CardComparer::remove_duplicated_card(&mut duplicated_cards);

        removed_repeating_duplicates.sort_by(|a,b| b.cmp(&a));

        return removed_repeating_duplicates
    }
}


#[cfg(test)]
mod card_comparer_tests {
    use super::*;

    fn push_all_cards<'a>(card1: &'a Card, card2: &'a Card, card3: &'a Card, card4: &'a Card, card5: &'a Card, card6: &'a Card, card7: &'a Card) -> Vec<&'a Card> {
        let mut cards: Vec<&Card> = Vec::new();
        cards.push(card1);
        cards.push(card2);
        cards.push(card3);
        cards.push(card4);
        cards.push(card5);
        cards.push(card6);
        cards.push(card7);

        cards
    }

     // CHECK FOUR OF A KIND

     #[test]
     fn check_four_a_kind_with_four_jacks_should_return_hands_four_of_a_kind() {
         let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
         let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
         let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
         let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
 
         let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);
 
         assert_eq!(
             CardComparer::check_four_of_a_kind(&mut cards), 
             Hands::FourOfAKind(Figure::Jack)
         );
     }

     #[test]
     fn check_four_a_kind_with_none_four_should_return_hands_none() {
         let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
         let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
         let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
         let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
 
         let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);
 
         assert_eq!(
             CardComparer::check_four_of_a_kind(&mut cards), 
             Hands::None
         );
     }

     #[test]
     fn check_four_a_kind_with_four_5_should_return_hands_four_of_a_kind() {
         let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
         let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
         let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
         let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
         let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Five};
 
         let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);
 
         assert_eq!(
             CardComparer::check_four_of_a_kind(&mut cards), 
             Hands::FourOfAKind(Figure::Five)
         );
     }

    // CHECK FULL HOUSE

    #[test]
    fn check_full_house_with_three_8_shouldnt_return_hands_full_house() {
        let card1: Card = Card {suit: Suit::Clubs, figure: Figure::Queen};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card3: Card = Card {suit: Suit::Spades, figure: Figure::Eight};
        let card4: Card = Card {suit: Suit::Diamonds, figure: Figure::Eight};
        let card5: Card = Card {suit: Suit::Spades, figure: Figure::Seven};
        let card6: Card = Card {suit: Suit::Diamonds, figure: Figure::Three};
        let card7: Card = Card {suit: Suit::Diamonds, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_ne!(
            CardComparer::check_full_house(&mut cards), 
            Hands::FullHouse(Figure::Eight)
        );
    }

    #[test]
    fn check_full_house_with_two_5_and_three_jacks_should_return_hands_full_house() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Five};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_full_house(&mut cards), 
            Hands::FullHouse(Figure::Jack)
        );
    }

    #[test]
    fn check_full_house_with_three_8_and_three_4_should_return_hands_full_house() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Eight};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Eight};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_full_house(&mut cards), 
            Hands::FullHouse(Figure::Eight)
        );
    }

    #[test]
    fn check_full_house_with_two_7_and_four_2_should_return_hands_full_house() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Seven};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_full_house(&mut cards), 
            Hands::FullHouse(Figure::Two)
        );
    }

    #[test]
    fn check_full_house_with_two_7_and_three_2_should_return_hands_full_house() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Seven};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_full_house(&mut cards), 
            Hands::FullHouse(Figure::Two)
        );
    }

    #[test]
    fn check_full_house_with_two_5_and_two_jacks_should_return_hands_none() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Five};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_full_house(&mut cards), 
            Hands::None
        );
    }

    // CHECK FLUSH

    #[test]
    fn check_flush_five_hearts_with_ace_should_return_hands_flush() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Seven};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Three};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_flush(&mut cards), 
            Hands::Flush(Suit::Hearts)
        );
    }

    #[test]
    fn check_flush_five_clubs_with_jack_should_return_hands_flush() {
        let card1: Card = Card {suit: Suit::Clubs, figure: Figure::Ten};
        let card2: Card = Card {suit: Suit::Clubs, figure: Figure::Jack};
        let card3: Card = Card {suit: Suit::Diamonds, figure: Figure::Eight};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card5: Card = Card {suit: Suit::Clubs, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Clubs, figure: Figure::Four};
        let card7: Card = Card {suit: Suit::Clubs, figure: Figure::Three};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_flush(&mut cards), 
            Hands::Flush(Suit::Clubs)
        );
    }

    // CHECK STRAIGHT

    #[test]
    fn check_straight_straight_from_jack_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Three};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Jack)
        );
    }

    #[test]
    fn check_straight_straight_from_ace_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Ace)
        );
    }

    #[test]
    fn check_straight_straight_from_6_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Six)
        );
    }

    #[test]
    fn check_straight_straight_from_6_with_pairs_of_2_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Six)
        );
    }

    #[test]
    fn check_straight_straight_from_9_with_pairs_of_2_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Two};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Nine)
        );
    }

    #[test]
    fn check_straight_straight_from_9_with_pairs_of_kings_should_return_hands_straight() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Nine};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Eight};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::King};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_straight(&mut cards), 
            Hands::Straight(Figure::Nine)
        );
    }

    // CHECK THREE OF A KIND

    #[test]
    fn check_three_of_a_kind_three_kings_should_return_hands_three_of_a_kind() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Six};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_three_of_a_kind(&mut cards), 
            Hands::ThreeOfAKind(Figure::King)
        );
    }

    #[test]
    fn check_three_of_a_kind_two_kings_and_four_six_should_return_hands_three_of_a_kind() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Six};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_three_of_a_kind(&mut cards), 
            Hands::ThreeOfAKind(Figure::Six)
        );
    }

    #[test]
    fn check_three_of_a_kind_three_jacks_should_return_hands_three_of_a_kind() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Jack};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_three_of_a_kind(&mut cards), 
            Hands::ThreeOfAKind(Figure::Jack)
        );
    }

    // CHECK TWO PAIRS

    #[test]
    fn check_two_pairs_two_pairs_2_and_6_should_return_hands_two_pairs() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Six, Figure::Two)
        );
    }

    #[test]
    fn check_two_pairs_three_4_and_three_10_should_return_hands_two_pairs() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Ten, Figure::Four)
        );
    }

    #[test]
    fn check_two_pairs_four_kings_and_two_aces_should_return_hands_two_pairs() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_two_pairs(&mut cards), 
            Hands::TwoPairs(Figure::Ace, Figure::King)
        );
    }

    // CHECK ONE PAIR

    #[test]
    fn check_one_pair_only_one_pair_of_4_should_return_hands_one_pair() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Four)
        );
    }
    
    #[test]
    fn check_one_pair_four_kings_should_return_hands_one_pair_kings() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Two};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Four};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::King)
        );
    }

    #[test]
    fn check_one_pair_three_kings_and_two_aces_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Three};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_two_aces_and_three_kings_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::King};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::King};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_two_aces_and_two_6_and_two_queens_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Five};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    #[test]
    fn check_one_pair_four_10_and_two_aces_should_return_hands_one_pair_ace() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_one_pair(&mut cards), 
            Hands::OnePair(Figure::Ace)
        );
    }

    // CHECK HIGH CARD

    #[test]
    fn check_high_card_ace_should_be_highest_card() {
        let card1: Card = Card {suit: Suit::Hearts, figure: Figure::Ten};
        let card2: Card = Card {suit: Suit::Hearts, figure: Figure::Four};
        let card3: Card = Card {suit: Suit::Hearts, figure: Figure::Six};
        let card4: Card = Card {suit: Suit::Hearts, figure: Figure::Seven};
        let card5: Card = Card {suit: Suit::Hearts, figure: Figure::Queen};
        let card6: Card = Card {suit: Suit::Hearts, figure: Figure::Ace};
        let card7: Card = Card {suit: Suit::Hearts, figure: Figure::Three};

        let mut cards: Vec<&Card> = push_all_cards(&card1, &card2, &card3, &card4, &card5, &card6, &card7);

        assert_eq!(
            CardComparer::check_high_card(&mut cards), 
            Hands::HighCard(Figure::Ace)
        );
    }
}