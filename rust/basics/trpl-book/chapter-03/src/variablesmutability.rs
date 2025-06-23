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

pub fn shadowing( ) {
    let x = 5;
    let x = x+1;
    {
        let x = x* 2;
        println!("{x}");
    }
    println!("{x}");
    // easy to change types : 
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");



    //tricky one
    // let mut x: u32 = 1;
    let x: u32 = 1;
    {
        let mut x = x;
        x += 2;
        println!("trickyone1: {x}");
    }
    println!("tricyone2:{x}");

}

pub fn mutability(){
    let mut x = 7;
    println!("{x}");
    x = 10;
    println!("{x}");
}

pub fn reference(){
    let mut x = 5;
    {

        let y = & x;
        println!("{y}");
        let y = &mut x;
        *y = 1;
        println!("{y}");
        let y = &*y+1;
        println!("{y}");
    }
    println!("{x}");
}