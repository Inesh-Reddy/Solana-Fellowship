pub fn variables() {
    // let x = 5 ;
    // println!("the value of x is : {x}");
    // x = 6;
    // println!("the value of x is : {x}");
    // // cannot assign twice to immutable variable `x` 

    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("the value of x is: {x}");
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    // println!("{}", THREE_HOURS_IN_SECONDS);
}