// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
use loops;

pub fn args_fn(value: &str) {
    // Either use 'cargo run' or 'rustc src/basics.rs && ./basics'
    println!("Hello, {}", value);
    if !value.contains("World!!") {
        // https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
        loops::for_loop(0)
    }
}
