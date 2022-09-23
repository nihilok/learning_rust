use crate::utility_funcs;

pub fn run() {
    let number = utility_funcs::string_to_usize(utility_funcs::read_line());
    if number > 5 {
        println!("{} is less than ", number);
    }
}