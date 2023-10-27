/*
Format specifiers are used to control how data is displayed or formatted as strings.
 */

mod number_systems;
mod alignment;

fn main() {
    default();
    numbered();
    debug();
    precision();
    pretty_print_debug();
    numbered_systems_trigger();
    aligned_print()
}

fn default() {
    let variable = "World";
    println!("Hello, {}", variable)
}

fn numbered() {
    let var1 = "Hello";
    let var2 = "World";
    println!("{0}, {1}", var1, var2)
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
        // key: "One".to_string(),
        key: String::from("One"),
        value: 1,
    };
    // placeholder to avoid the 'dead_code' warning (default)
    //  this can also be overridden by setting '#[allow(dead_code)]'
    println!("Key: {}, value: {}", json_data.key, json_data.value);
    println!("Debug with Pretty-Print: {:#?}", json_data);
}


fn numbered_systems_trigger() {
    number_systems::hex();
    number_systems::binary();
    number_systems::octal();
}

fn aligned_print() {
    alignment::left_aligned();
    alignment::center_aligned();
    alignment::right_aligned();
}
