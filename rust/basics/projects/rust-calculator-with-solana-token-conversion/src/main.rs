mod operations;
use std::{io};
use operations::{addition, subtraction, multiplication, division};
fn main() {
    println!("Solana Calculator");

    let mut op = String::new();
    let mut a = String::new();
    let mut b =String::new();

    println!("Enter operation (add, sub, mul, div):");
    io::stdin().read_line(&mut op).unwrap();

    println!("Enter first number:");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter second number:");
    io::stdin().read_line(&mut b).unwrap();

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let op = op.trim();

    let mut ans:f64= 0.0;

    if op == "add" {
       ans = addition(a, b);
       println!("Result : {}",ans);
    } else if op == "sub" {
        ans = subtraction(a, b);
        println!("Result : {}", ans);
    } else if op == "mul" {
        ans = multiplication(a, b);
        println!("Result : {}", ans);
    } else if op == "div" {
        ans = division(a, b);
        println!("Result : {}", ans);
    } else {
        println!("Please enter a valid input");
    }
    
}
