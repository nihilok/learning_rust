use crate::{player_turn, utility_funcs};
use crate::player_turn::Player;

const PLAYER_BUY_IN: i32 = 5;
const PLAYER_STARTING_CASH: i32 = 5;


pub fn run() {
    println!("How many players?");
    let num_players: i32 = loop {
        let n = utility_funcs::get_user_int_input();
        if n > 0 {
            break n;
        }
    };
    utility_funcs::clear_terminal();
    let mut pool: i32 = num_players * PLAYER_BUY_IN;
    let p = Player {
        player_score: PLAYER_STARTING_CASH,
        player_number: 1,
    };
    let mut players: Vec<Player> = vec![p];
    if num_players > 1 {
        for player in 2..=num_players {
            let p = Player {
                player_score: PLAYER_STARTING_CASH,
                player_number: player,
            };
            players.push(p)
        }
    }
    loop {
        for mut player in players.iter() {
            println!("Player {}:\n", player.player_number);
            println!("You currently have ${}", player.player_score);
            println!("The pot is currently ${pool}\n");
            let player_score = player.player_score;
            let round_result = player_turn::run(&mut pool, player_score);
            let new_player = Player {
                player_number: player.player_number,
                player_score: round_result.1,
            };
            player = &new_player;
            pool = round_result.0;
            if player.player_score > 0 {
                println!("You now have ${}", player.player_score);
            } else {
                println!("You now have ${}", player.player_score);
            }
            if pool == 0 {
                println!("Player {} won the pot! Game over!", player.player_number);
                return;
            }
            println!("Pot is now ${pool}\n");
            println!("Player {}, now your turn!\nPress enter to continue...", if player.player_number < num_players { player.player_number + 1 } else { 1 });
            let _ = utility_funcs::read_line();
            utility_funcs::clear_terminal();
        }
    }
}