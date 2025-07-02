/*
    if
 */

 pub fn iffing() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
 }

 pub fn ifelseing() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("numnber is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("Numnber is divisible by 1");
    }
 }
 