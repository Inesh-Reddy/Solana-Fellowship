pub fn owner2() {
    let name = String::from("Rust Ownership");
    let (name, returned_name) = take_modify_and_return(name);
    println!("given name: {name} and returned name:{returned_name}");

}

fn take_modify_and_return(s: String)-> (String, String) {
    let received_name = s.clone();
    let modified_name = String::from(received_name+" modified");
    (s,modified_name)
}