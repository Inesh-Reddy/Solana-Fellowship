use crate::{datatypes::arrays, variablesmutability::{mutability, reference, shadowing, variables}};

mod variablesmutability;
mod datatypes;

fn main() {

    
    println!("Hello, world!");
    println!("------- variabels -------");
    variables();
    println!("------- shadowing -------");
    shadowing();
    println!("------- mutability -------");
    mutability();
    println!("------- referencing -------");
    reference();


    arrays();
}
