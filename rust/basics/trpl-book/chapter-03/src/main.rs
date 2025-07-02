use crate::{datatypes::arrays, ifelse::{ifelseing, iffing}, loops::{looping, nestedloopingwithlabels}, variablesmutability::{mutability, reference, shadowing, variables}};

mod variablesmutability;
mod datatypes;
mod ifelse;
mod loops;

fn main() {

    
    println!("Hello, world!");
    // println!("------- variabels -------");
    // variables();
    // println!("------- shadowing -------");
    // shadowing();
    // println!("------- mutability -------");
    // mutability();
    // println!("------- referencing -------");
    // reference();
    // arrays();
    println!("------- conditions -------");
    // iffing();
    // ifelseing();
    // looping();
    let result  = nestedloopingwithlabels();
    println!("{result}");


    
}
