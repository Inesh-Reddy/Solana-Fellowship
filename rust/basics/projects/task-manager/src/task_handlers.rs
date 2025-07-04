use crate::{state::TASKS};

pub fn adding(task_to_add: String) -> String {
    println!("🧪 adding task .....");
    let cleaned = task_to_add.trim().to_string(); 
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(format!("{} : ❌",cleaned.clone()));
    let to_return = format!("✅ Task: {} added successfully", cleaned);
    to_return
}

pub fn viewing() -> Vec<String> {
    println!("💭 ");
    let tasks = TASKS.lock().unwrap();
    tasks.clone()
}

pub fn completing(todo_to_modify:usize)-> String {
    println!("called modify function");
    let mut task = TASKS.lock().unwrap();
    if (todo_to_modify as usize) >= task.len() {
        return "❌ Please provide a proper todo number.".to_string();
    }else {
        task[todo_to_modify as usize] = format!("{} : ✅ ", task[todo_to_modify as usize]);
        return "✅ task modified".to_string();
    }
}

pub fn removing_tasks(todo_to_delete:usize) -> String {
    println!("called DELETE function");
    let mut task = TASKS.lock().unwrap();
    if (todo_to_delete as usize) >= task.len() {
        return "❌ Please provide a proper todo number to DELETE.".to_string();
    }else {
        task.remove(todo_to_delete);
        return "✅ task DELETED".to_string();
    }
}