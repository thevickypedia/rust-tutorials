use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let arg_count = arguments.len();
    if arg_count == 2 {
        let var = &arguments[1];  // type: &String | &str
        args(var)
    } else {
        args("World!!");  // type: &str
    }
}

fn args(value: &str) {
    // Either use 'cargo run' or 'rustc src/basics.rs && ./basics'
    println!("Hello, {}", value);
    if !value.contains("World!!") {
        println!("Heading to loops");
        looping()
    }
}


fn looping() {
    for i in [1, 2, 3, 4] {
        println!("{}", i);
    }
}
