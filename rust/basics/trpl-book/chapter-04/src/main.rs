use crate::{ownership::owner, take_and_give::{owner2}};

mod ownership;
mod take_and_give;
fn main() {
    println!("Hello, world!");
    owner();
    owner2();
}
