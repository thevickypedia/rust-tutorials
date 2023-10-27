fn main() {
    default();
    debug();
    precision()
    // todo: implement pretty print for debug
}

fn default() {
    let variable = "World";
    println!("Hello, {}", variable)
}

fn debug() {
    let numbers = vec![1, 2, 3, 4];
    println!("Debug: {:?}", numbers)
}

fn precision() {
    let pi = std::f64::consts::PI;
    println!("Pi: {:.2}", pi);  // limits to 2 decimal points (3.14)
}
