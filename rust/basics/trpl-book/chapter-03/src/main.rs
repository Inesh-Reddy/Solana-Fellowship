use crate::variablesmutability::{mutability, reference, shadowing, variables};

mod variablesmutability;

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
}
