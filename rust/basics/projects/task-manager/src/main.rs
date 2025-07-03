use std::io::{self, Write};

use crate::task_handlers::{adding, viewing};
mod task_handlers;
mod state;
fn main() {
    println!("Task - Manager");
    
    

    'outer_loop :loop {
        loop{
            let mut input_string = String::new();
            println!();
            println!("---------------------------------");
            println!();
            println!("1. Add a task");
            println!("2. View all tasks");
            println!("3. Mark a task as completed");
            println!("4. Remove a taks");
            println!("5. Exit");
            println!();
            println!("Please select the operation: ");
    
            std::io::stdin().read_line(&mut input_string).expect("Error!!");
            let input = input_string.trim();
    
            if input == "1" {
                let mut task_to_add = String::new();
                println!("Please provide task to add:");
                std::io::stdin().read_line(&mut task_to_add).expect("Please provide String");
                let result = adding(task_to_add);
                io::stdout().write_all(result.as_bytes()).expect("Write failed");
                break;
            }else if input=="2" {
                let result = viewing();
                for i in 0..result.len() {
                    println!("✅ Task {}: {} ", i+1,result[i]);
                }
            }else if input=="3" {
                
            }else if input=="4" {
                
            }else if input=="5" {
                println!("Thank you please visit again!!");
                break 'outer_loop ;
            } else {
                println!("Please provide proper input");
            }
        }
    }

}
