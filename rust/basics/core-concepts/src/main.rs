mod is_even;
mod fib;
mod get_string_length;
mod structs;

use structs::user_details;

fn main() {
    println!("Hello, world!");
    println!("{}", is_even::is_even(10));
    println!("{}", fib::fib(4));
    println!("{}", get_string_length::get_string_length("Rust lings".to_string()));
    println!("User details : {:?}", user_details());
    let user = user_details();
    println!("first_name : {:?}", user.first_name);
}
