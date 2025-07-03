use crate::state::TASKS;

pub fn adding(task_to_add: String) -> String {
    println!("🧪 adding task .....");
    let cleaned = task_to_add.trim().to_string(); 
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(cleaned.clone());
    let to_return = format!("✅ Task: {} added successfully", cleaned);
    to_return
}

pub fn viewing() -> Vec<String> {
    println!("💭 Fetching tasks....");
    let mut tasks = TASKS.lock().unwrap();
    tasks.clone()
}

pub fn completing() {

}

pub fn removing_tasks() {

}