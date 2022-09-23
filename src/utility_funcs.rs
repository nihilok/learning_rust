use std::io;

pub fn read_line() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    return buf
}

pub fn string_to_int(str_input: String) -> i32 {
    return str_input.trim().parse::<i32>().expect("Not a number");
}

pub fn string_to_usize(str_input: String) -> usize {
    return str_input.trim().parse::<usize>().expect("Not a number");
}

