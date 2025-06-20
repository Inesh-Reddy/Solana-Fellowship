
#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    pub first_name: String,
    last_name: String,
    email: String,
    age: i32,
}


pub fn user_details() -> User{
    let user = User {
        first_name: String::from("Harkirath"),
        last_name: String::from("Singh"),
        email: String::from("hello123@gmail.com"),
        age:32
    };
    return user;

}