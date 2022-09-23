use std::io;
use rand::Rng;

pub fn read_line() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    return buf;
}

pub fn string_to_int(str_input: String) -> i32 {
    return str_input.trim().parse::<i32>().expect("Not a number");
}

pub fn string_to_usize(str_input: String) -> usize {
    return str_input.trim().parse::<usize>().expect("Not a number");
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

pub fn get_user_int_input() -> i32 {
    return string_to_int(read_line())
}

pub fn get_user_usize_input() -> usize {
    return string_to_usize(read_line())
}

pub const fn string_to_usize_const(str_input: String) -> usize {
    return str_input.trim().parse::<usize>().expect("Not a number");
}

pub const fn get_user_usize_input_const() -> usize {
    const INPUT: String = read_line();
    return string_to_usize_const(INPUT)
}

pub fn roll_dice() -> i32 {
    return rand::thread_rng().gen_range(1..=6)
}