// src/state.rs
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    // This makes a global, mutable, thread-safe task list
    pub static ref TASKS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    // pub static TASKS2: Vec<String> = Vec::new();
}

