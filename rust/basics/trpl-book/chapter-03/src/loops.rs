use core::num;

pub fn looping() {
    let mut number = 0 ;
    loop {
        println!("value of numner is {}", number);

        if number == 10 {
            break;
        } else {
            number = number + 1;
        }
    }
    println!("The end");

    let mut num: i8 = 1;
    loop {
        println!("value is {}", num);

        if num == 0 {
            break;
        } else {
            num = num * 2;
        }
    }
    println!("the second end")
}

pub fn nestedloopingwithlabels() -> i32 {
    let mut num = 1;

    'outer_loop: loop {
        if num == 10 {
           return num+1;
        }
        loop {
            if num == 5 {
                break 'outer_loop num-1;
            }
            break;
        }
        num = num + 1;
    }
}