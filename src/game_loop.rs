use crate::{player_turn, utility_funcs};
use crate::player_turn::Player;

const PLAYER_BUY_IN: i32 = 5;
const PLAYER_STARTING_CASH: i32 = 5;
const RULES: &str = "On each turn a die is rolled. After the first number is revealed, \
before the second die is rolled, you must make a bet depending on how likely you think \
it is that the next roll of the die will be lower than the first. If the first die is 1, \
you miss a turn.";

fn handle_round_result(result: (i32, i32), player_number: i32) -> (i32, Player) {
    let new_player = Player {
        player_number,
        player_score: result.1,
    };
    let pool = result.0;
    if result.1 > 0 {
        println!("You now have ${}", new_player.player_score);
    } else {
        println!("You now have ${}", new_player.player_score);
    }
    return (pool, new_player);
}

fn rules() {
    println!("{}", RULES);
    println!("Press enter to continue...");
    utility_funcs::read_line();
    utility_funcs::clear_terminal();
}

fn next_turn(num_players: i32, player_index: usize, players: &Vec<i32>) {
    let p_u: usize = player_index as usize + 1;
    println!("Player {}, now your turn!\nPress enter to continue...", if p_u < num_players as usize { players[p_u] } else { players[0] });
    utility_funcs::read_line();
    utility_funcs::clear_terminal();
}

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
    rules();
    loop {
        let mut next_round_players: Vec<Player> = vec![];
        let players_this_round: i32 = players.len() as i32;
        let mut player_indices: Vec<i32> = vec![];
        for (i, player) in players.iter().enumerate() {
            for player in players.iter() {
                player_indices.push(player.player_number)
            }
            println!("Player {}:\n", player.player_number);
            println!("You currently have ${}", player.player_score);
            println!("The pot is currently ${pool}\n");
            let round_result = player_turn::run(&mut pool, player.player_score);
            let (pool, player) = handle_round_result(round_result, player.player_number);
            if pool == 0 {
                println!("Player {} won the pot! Game over!", player.player_number);
                return;
            }
            println!("Pot is now ${pool}\n");
            if player.player_score == 0 {
                println!("Player {} is out!", player.player_number);
                next_turn(players_this_round, i, &player_indices);
                continue;
            }
            next_turn(players_this_round, i, &player_indices);
            if player.player_score > 0 {
                next_round_players.push(player);
            }
        }
        players = next_round_players;
    }
}