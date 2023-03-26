pub fn simple_multiplication(number: u8) -> u8 {
    let odd = number & 1 == 1;
    if odd {
        return number * 9;
    } else {
        return number * 8;
    }
}
