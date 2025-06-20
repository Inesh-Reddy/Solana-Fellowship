#[derive(Debug)]

pub struct Rect {
    pub width: i32,
    pub height: i32
}

impl Rect {
    pub fn area(&self) -> i32 {
        &self.width * &self.height
    }

    pub fn perimeter(&self) -> i32 {
        2 * (&self.width + &self.height)
    }

    pub fn debug() -> String {
        return String::from("Hello : line1");
    }
}


