fn main() {
    println!("The Rust Programming language Book : chapter 2 ");

    println!("Guess the number!");
    println!("Please input your guess : ");

    let mut guess = String::from("");

    std::io::stdin()
        .read_line( &mut guess)
        .expect("you got some error");

    println!("You guessed: {guess}");

    

}
