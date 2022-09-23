use std::fs;

pub fn main() {
    let file_path = "./src/dice.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    print!("{}", contents)
}