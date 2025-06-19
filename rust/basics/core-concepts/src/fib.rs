// Write a function fib that finds teh fibbonacci of a number it takes as input 

pub fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _i in 1 .. (num - 1){
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;

}