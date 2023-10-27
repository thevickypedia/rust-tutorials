use std::env;

use args;

pub fn entrypoint() {
    let arguments: Vec<String> = env::args().collect();
    let arg_count = arguments.len();
    println!("{}", arg_count);
    if arg_count >= 2 {
        let var = &arguments.last().unwrap().to_string();  // type: &String | &str
        args::args_fn(var)
    } else {
        args::args_fn("World!!");  // type: &str
    }
}
