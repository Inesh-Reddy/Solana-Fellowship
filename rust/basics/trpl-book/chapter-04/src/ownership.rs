pub fn owner() {
    let name = String::from("Rust language");
    let x = 20;
    let result = add(x);
    println!("{result}");
    println!("value of name: {}", name);
    takes_ownership(name);
}
fn takes_ownership(s:String){
    println!("value of s: {}",s);
}
fn add(x:i32)-> i32 {
    x+10
}

