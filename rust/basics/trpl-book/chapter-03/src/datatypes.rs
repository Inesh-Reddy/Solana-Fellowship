// undestading arrays
use std::io;
pub fn arrays(){
    let a:[i32; 7]= [1,2,3,4,5,6,7];
    println!("please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("failed to read");

    let getelement = a[index];
    println!("the value of the elemetn at index {index} is : {getelement}");

}