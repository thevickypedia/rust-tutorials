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
        // https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
        looping(0)
    }
}


fn looping(skip_val: usize) {
    // .skip will skip the index value in the vector (not the index when enumerating)
    // skipping 0 means, nothing will be skipped
    // skipping 1 means, 'a' will be skipped and so on....
    for (i, v) in ['a', 'b', 'c', 'd'].iter().enumerate().skip(skip_val) {
        println!("[{}] {}", i, v);
    }
}
