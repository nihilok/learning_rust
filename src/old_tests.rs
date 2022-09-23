use crate::utility_funcs;

pub fn first_trials() {
        // i8
    let difference: i8 = 127 - 31;
    let difference2: i8 = 127 - 21;
    let result = i8::from(difference - difference2);

    // Tuple
    let tup = (difference, difference2, result, "chars");
    let (x, y, z, str) = tup;
    let w = tup.2;
    println!("{}", w);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", result);
    println!("{}", str);

    // Array
    let arr = [x, y, z];
    println!("{:?}", arr);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("1st month: {}\n12th month: {}", months[0], months[11]);

    // Array with 10 of the same value
    let same_arr = [x; 10];
    println!("first x: {}\nlast x: {}", same_arr[0], same_arr[9]);

    let str = utility_funcs::read_line();
    println!("{}", str);

    let index = utility_funcs::string_to_usize(str);
    println!("{}", months[index - 1]);
}