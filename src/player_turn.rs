use crate::utility_funcs;

pub fn run(pool: &mut i32) -> Tuple {
    utility_funcs::clear_terminal();
    let mut bet = 0;
    bet = loop {
        let throw_1 = utility_funcs::roll_dice();
        let throw_2 = utility_funcs::roll_dice();
        println!("First throw was {}", throw_1);
        println!("How much do you bet that the next throw is lower than {}?", throw_1);
        bet = utility_funcs::get_user_int_input();
        println!("Second throw was {}", throw_2);
        if throw_1 > throw_2 {
            println!("Throw 2 was lower... you win!");
            break if bet <= pool {
                pool -= bet;
                bet * 2
            } else {
                pool = 0;
                bet + pool
            }
        } else if throw_1 < throw_2 {
            println!("Throw 2 was higher... you lose!");
            break 0;
        }
        utility_funcs::clear_terminal();
        println!("Throws were equal! Try again!\n\n")
    };
    return bet;
}