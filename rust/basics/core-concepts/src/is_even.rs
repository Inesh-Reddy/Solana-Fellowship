// Write a function is_even that takes a number as an input adn returns true if it is even

pub fn is_even(n: i32) -> &'static str {
    if n % 2 == 0 { "even" } else { "odd" }
}
