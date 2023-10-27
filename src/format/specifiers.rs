fn main() {
    default();
    debug();
    precision();
    pretty_print_debug();
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

// Debug annotation for JSONData
#[derive(Debug)]
struct JSONData {
    key: String,
    value: u32,
}

fn pretty_print_debug() {
    let json_data = JSONData {
        key: "One".to_string(),
        value: 1,
    };
    // placeholder to avoid the 'dead_code' warning (default)
    //  this can also be overridden by setting '#[allow(dead_code)]'
    println!("Key: {}, value: {}", json_data.key, json_data.value);
    println!("Debug with Pretty-Print: {:#?}", json_data);
}
