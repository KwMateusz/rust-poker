use crate::utils::card::Card;
use crate::utils::player::Player;
use crate::utils::hands::*;

#[derive(Debug)]
pub struct Table {
    pub flop: Option<(Card, Card, Card)>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
    pub pot: u32,
    pub to_call: u32,
}

impl Table {
    pub fn new() -> Table {
        Table {
            flop: None,
            turn: None,
            river: None,
            pot: 0,
            to_call: 0,
        }
    }

    pub fn is_clean(&self) -> bool {
        self.flop == None && self.flop == None && self.flop == None && self.pot == 0
    }

    pub fn clean(&mut self) {
        self.flop = None;
        self.turn = None;
        self.river = None;
        self.pot = 0;
        self.to_call = 0;
    }

    pub fn set_pot(&mut self, cash_to_pot: u32) {
        self.pot += cash_to_pot;
    }

    pub fn compare_cards<'a>(&mut self, players: &'a mut Vec<&'a mut Player>) {
        if let Some((card1, card2, card3)) = &self.flop {
            if let Some(card4) = &self.turn {
                if let Some(card5) = &self.river {
                    let card1_clone = card1.clone();
                    let card2_clone = card2.clone();
                    let card3_clone = card3.clone();
                    let card4_clone = card4.clone();
                    let card5_clone = card5.clone();
                    CardComparer::get_better_hands(players, card1_clone, card2_clone, card3_clone, card4_clone, card5_clone);
                } 
            } 
        } 
        
        //println!("{:?}", self);
        //println!("Players[0] hand{:?}", players[0].hand);
        //println!("Players[1] hand{:?}", players[1].hand);
        //println!("")

        //players.pop().unwrap() // only for return purpose
    }

    pub fn collect_reward(&mut self, player: &mut Player) {
        player.money = self.pot;
    }
}
