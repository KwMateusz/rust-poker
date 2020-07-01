use crate::utils::card::Card;
use crate::utils::util::*;

pub const STARTING_MONEY: u32 = 15000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Bet(u32),
    Pass,
    Check,
    Lost,
    None,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Option<(Card, Card)>,
    pub money: u32,
    pub status: Status,
}

impl Player {
    pub fn new(name: String) -> Player {
        println!("[Player] {} joined the game.", name);

        Player {
            name,
            hand: None,
            money: STARTING_MONEY,
            status: Status::None,
        }
    }

    pub fn new_round(&mut self) {
        self.hand = None;
        self.reset_status();
    }

    pub fn reset_status(&mut self) {
        self.status = Status::None;
    }

    pub fn show_cards(&self) {
        if let Some((first, second)) = &self.hand {
            println!("Your cards: {}\t {}", first, second);
        } else {
            println!("Could not show the cards");
        }
    }

    pub fn bet(&mut self, cash_amount: u32) {
        println!("{} is beting {} dollars.", self.name, cash_amount);
        self.money -= cash_amount;
    }

    pub fn make_decision(&mut self, to_call: &mut u32) {
        println!("***** Player {} makes action *****", self.name);
        self.show_cards();

        'decision: loop {
            println!("What is your decision?\nBET\tCHECK\tPASS\tCALL");
            match &*read_line().trim().to_uppercase() {
                "BET" => {
                    'bet: loop {
                        println!("How much do you want to BET? ");
                        let bet = read_line().trim().parse::<u32>().unwrap();
                        if bet > self.money { 
                            println!("You BET more than you have!");
                        } else {
                            if bet != 0 { 
                                if bet <= *to_call{
                                    println!("BET must be higher than {}!", to_call);
                                    println!("You sure you want to BET? Y / N");
                                    if read_line().trim() == "N" { 
                                        continue 'decision;
                                    }
                                }else {
                                    self.status = Status::Bet(bet);
                                    *to_call = bet;
                                    break 'decision;
                                }  
                            }
                        }
                    }
                }
                "CHECK" => {
                    if *to_call == 0 {
                        self.status = Status::Check;
                        break 'decision; 
                    } else {
                        println!("You can't CHECK");
                    }
                }
                "PASS" => {
                    self.status = Status::Pass;
                    break 'decision;
                }
                "CALL" => {
                    if self.money < *to_call {
                        println!("You can't CALL, you don't have that much money!");
                        continue 'decision;
                    }
                    self.status = Status::Bet(*to_call);
                    break 'decision;
                }
                _ => {
                    println!("Incorrect decision!");
                }
            }// match
        }// decision loop
    }// fn make_decision
}