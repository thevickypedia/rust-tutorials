use std::env;

fn main() {
    println!("Hello, World!");
    args()
}

fn args() {
    // Either use 'cargo run' or 'rustc src/basics.rs && ./basics'
    println!("\nCommandLine argument section");
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    println!("Invoker: '{}'", program_name);
    looping()
}


fn looping() {
    println!("\nLooping section");
    for i in [1, 2, 3, 4] {
        println!("{}", i);
    }
}
