mod utils;

use crate::utils::game::Game;
use crate::utils::util::*;

use crate::utils::hands::*;
use crate::utils::card::*;

fn main() {
    println!("Hello in Poker Rust! \nWould you like to start? Y / N");
    match read_line().trim() {
        "Y" => (),
        "N" => { 
            println!("Thank you for your time! :)");
            std::process::exit(0);
        },
        _ => panic!("Wrong input!"),
    }
        
    let mut game: Game = Game::new();
    game.add_players();
    println!("All players are ready! Let's start the game!\n");
    game.run();
    println!("Thank you for your time! :)");
}