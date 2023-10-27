pub fn hex() {
    // x and X format specifiers are specifically used for hexadecimal formatting
    let number = 255;
    println!("Hex number: {}", number.to_string());
    println!("Hexadecimal value: {:x}, Uppercase Hex value: {:X}", number, number);
}

pub fn binary() {
    // b format specifier is specifically used for binary formatting
    let number = 255;
    println!("Binary number: {}", number.to_string());
    println!("Binary value: {:b}", number);
}

pub fn octal() {
    // o format specifier is specifically used for octal formatting
    let number = 255;
    println!("Octal number: {}", number.to_string());
    println!("Octal value: {:o}", number);
}
