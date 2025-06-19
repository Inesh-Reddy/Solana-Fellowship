// Write a function get_String_Length that takes a string as an input and returns its lenght

pub fn get_string_length(input_string: String)-> i32 {
    let mut count = 0;
    for _i in 0..input_string.len()  {
        count = count+1;
    }
    return count;
}
