
pub fn addition(num1:f64, num2: f64) -> f64 {
    num1+num2

}

pub fn subtraction(num1:f64, num2:f64) -> f64 {
    num1-num2
}

pub fn division(num1:f64, num2:f64) -> f64 {
    if num2 == 0.0 {
        return 0.0;
    }
    return num1/num2;

}

pub fn multiplication(num1:f64, num2:f64) -> f64 {
    num1*num2
}