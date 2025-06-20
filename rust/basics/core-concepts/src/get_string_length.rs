// Write a function get_String_Length that takes a string as an input and returns its lenght
fn get_string_lenght_chars(string_toconvert: &str) -> usize {
    string_toconvert.chars().count()
}

pub fn get_string_length(input_string: String) -> usize {
    // let mut count = 0;
    // for _i in 0..input_string.len()  {
    //     count = count+1;
    // }
    let count = get_string_lenght_chars(&input_string);
    return count;
}
