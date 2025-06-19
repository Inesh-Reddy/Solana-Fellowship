mod is_even;
mod fib;

fn main() {
    println!("Hello, world!");
    println!("{}", is_even::is_even(10));
    println!("{}", fib::fib(4));
}
