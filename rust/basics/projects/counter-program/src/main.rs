use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, Read};


fn main() {
    println!("------Counter : Increment and decrement------");
    /*
    - Create a function called increment and decrement that reads from a file(a.bin), update the value of a counter value inside and rewrite it to the file .
*/
    // create_file("a");
    // let _ = read_file();
    // let _test = increment();
    // let _test2 = decrement();
    let _destroy = deltecontent();

}

fn create_file(name: &str) {
    let mut file = File::create(format!("./counter-program/src/{}.bin",name)).unwrap();
    file.write_all( b"0").unwrap();
}   
fn read_file() -> io::Result<()> {
    let path = "./counter-program/src/a.bin";

    let mut file = File::open(path).unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap(); 

    println!("File content: {:?}", content);

    Ok(())
}


fn increment() -> io::Result<()>{
    let path = "./counter-program/src/a.bin";
    let mut file = File::open(path).unwrap(); 
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap(); 
    let value = String::from_utf8(content).unwrap();
    let mut result:i32 = value.parse().unwrap();
    println!("value in file: {:?}", result);

    result = result+1;
    println!("increment: {:?}", result);
    let new_content = result.to_string();
    let mut file = File::create(path)?; 
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn decrement() -> io::Result<()>{
    let path = "./counter-program/src/a.bin";
    let mut file = File::open(path).unwrap(); 
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).unwrap(); 
    let value = String::from_utf8(content).unwrap();
    let mut result:i32 = value.parse().unwrap();
    println!("value in file: {:?}", result);

    result = result-1;
    println!("decrememnt: {:?}", result);
    let new_content = result.to_string();
    let mut file = File::create(path).unwrap(); 
    file.write_all(new_content.as_bytes()).unwrap();
    Ok(())
}

fn deltecontent(){
    let path = "./counter-program/src/a.bin";
    OpenOptions::new().write(true).truncate(true).open(path);
    println!("Cleared content form file.");
}





