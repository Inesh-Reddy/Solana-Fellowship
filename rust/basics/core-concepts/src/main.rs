mod is_even;
mod fib;
mod get_string_length;

mod structs;
mod implementing_structs;

use structs::user_details;
use implementing_structs::Rect;


fn main() {
    println!("Hello, world!");
    println!("{}", is_even::is_even(10));
    println!("{}", fib::fib(4));
    println!("{}", get_string_length::get_string_length("Rust lings".to_string()));
    println!("User details : {:?}", user_details());
    let user = user_details();
    println!("first_name : {:?}", user.first_name);

    let recta = Rect {
        width: 20,
        height: 30,
    };
    println!("area : {:?}", recta.area());
    println!("perimeter: {:?}", recta.perimeter());
    println!("debug: {}", Rect::debug());
}
