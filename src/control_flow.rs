use crate::utility_funcs;

pub fn run() {
    println!("Enter a number...");
    let number = utility_funcs::string_to_usize(utility_funcs::read_line());
    if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is less than 5", number);
    }
}
