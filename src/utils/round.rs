use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::utils::table::Table;
use crate::utils::deck::Deck;
use crate::utils::player::{Status, Player};

#[derive(Debug)]
pub struct Round {
    pub table: Table,
    pub deck: Deck,

}

impl Round {
    pub fn new() -> Round {
        Round {
            table: Table::new(),
            deck: Deck::new(),
        }
    }

    pub fn new_round(&mut self, players: &mut Vec<&mut Player>) {
        self.table = Table::new();
        self.deck = Deck::new();
        for p in players.iter_mut(){
            p.new_round();
        }
    }

    pub fn end_round(&mut self) {
        self.table.clean();
        self.deck.clean();
        println!("***** Round ends *****\n\n\n");
    }

    pub fn shuffle_cards(&mut self) {
        self.deck.cards.shuffle(&mut thread_rng());
    }

    pub fn show_cards(&self, round_number: u32) {
        let mut card_displayer: String = String::from("");

        match round_number {
            0 => card_displayer.push_str(&format!("\n===== Blind round =====\n")),
            1..=3 => {
                let (_1, _2, _3) = self.table.flop.as_ref().unwrap();
                card_displayer.push_str(&format!("\n\nCards on table:  [{}]\t[{}]\t[{}]", _1, _2, _3));
                if round_number > 1 {
                    let _4 = self.table.turn.as_ref().unwrap();
                    card_displayer.push_str(&format!("\t[{}]", _4));
                }
                if round_number > 2 {
                    let _5 = self.table.river.as_ref().unwrap();
                    card_displayer.push_str(&format!("\t[{}]", _5));
                }
                card_displayer.push_str(&format!("\n"))
            },
            _ => card_displayer.push_str(&format!("Round ended."))
        }
        
        println!("{}", card_displayer);
    }

    pub fn deal_cards(&mut self, players: &mut Vec<&mut Player>) {
        for p in players {
            if let None = p.hand {
                p.hand = Some((self.deck.take_card(), self.deck.take_card()));
                //println!("Croupier dealt cards to {}", p.name);
            } else {
                println!("Cards have already been dealt.\n");
            };
        }
        println!("");
        if self.table.is_clean() {
            self.table.flop = Some((self.deck.take_card(), self.deck.take_card(), self.deck.take_card()));
            self.table.turn = Some(self.deck.take_card());
            self.table.river = Some(self.deck.take_card());
        }
        //println!("Croupier dealt the cards.\n");
    }

    pub fn new_sub_round(&mut self, players: &mut Vec<&mut Player>){
        for p in players {
            p.reset_status();
        };
        self.table.to_call = 0;
    }

    pub fn run(&mut self, players_ptr: &mut Vec<&mut Player>) {
        let mut round_part_number: u32= 0; // 0: "Blinds", 1: "Flop", 2: "Turn", 3: "River"
        'round: while round_part_number < 4 && players_ptr.len() > 1 {
            self.new_sub_round(players_ptr);
            self.show_cards(round_part_number);

            let mut turn_index = 0;
            let mut turn_left = players_ptr.len() - 1;

            'inner: while turn_left != 0 {
                let playern_index = turn_index % players_ptr.len();
                players_ptr[playern_index].make_decision(&mut self.table.to_call);
                if let Status::Bet(x) = players_ptr[playern_index].status {
                    players_ptr[playern_index].bet(x);
                    self.table.set_pot(x);
                }
                if players_ptr[playern_index].status == Status::Pass {
                    players_ptr.remove(playern_index);
                    turn_left -= 1;
                    continue 'inner;
                } else if players_ptr[playern_index].status == 
                        players_ptr[(players_ptr.len() + playern_index - 1) % players_ptr.len()].status {
                    turn_left -= 1;
                } else {
                    turn_left = players_ptr.len() - 1;
                }
                turn_index += 1;
            }// inner loop
            round_part_number += 1;
            
        }// round loop
    }

    pub fn check_round_winner<'a>(&mut self, players: &'a mut Vec<&'a mut Player>) {
        if players.len() == 1 {
            self.table.collect_reward(players.pop().unwrap());
        } else {
            let winner = self.table.compare_cards(players);
            self.table.collect_reward(winner);
        }
    }
}