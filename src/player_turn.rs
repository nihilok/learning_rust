use crate::utility_funcs;

pub struct Player {
    pub player_number: i32,
    pub player_score: i32,
}

fn sort_last_throw(last_throw: i32) -> i32 {
    return if last_throw == 0 {
        let throw = utility_funcs::roll_dice();
        throw
    } else {
        last_throw
    };
}

pub fn run(pool: &mut i32, player_score: i32) -> (i32, i32) {
    let mut last_throw = 0;
    let new_score = loop {
        let throw_1 = sort_last_throw(last_throw);
        let throw_2 = utility_funcs::roll_dice();
        if throw_1 == last_throw {
            last_throw = 0;
        }
        println!("First throw was {}", throw_1);
        println!("How much do you bet that the next throw is lower than {}?", throw_1);
        let bet = utility_funcs::get_user_int_input();
        utility_funcs::clear_terminal();
        last_throw = throw_1;
        if bet == 0 {
            println!("Invalid bet. Please try again...\n");
            continue;
        }
        if bet > *pool || bet > player_score {
            let max_bet = if bet > player_score { player_score } else { *pool };
            println!("{bet} is greater than the maximum bet ({max_bet}). Please bet again...\n");
            continue;
        }
        last_throw = 0;
        println!("Second throw was {}", throw_2);
        if throw_1 > throw_2 {
            println!("YES! {throw_2} is lower than {throw_1}... you win!\n");
            if bet <= *pool {
                *pool -= bet;
                break bet * 2;
            } else {
                *pool = 0;
                break bet + *pool;
            }
        } else if throw_1 < throw_2 {
            println!("OH NO! {throw_2} is higher than {throw_1}... you lose!\n");
            *pool += bet;
            break 0;
        }
        println!("Throws were equal! Try again!\n")
    };
    return (*pool, player_score + new_score);
}