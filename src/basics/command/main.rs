extern crate chrono;

use std::env;
use std::process::{Command, exit};

fn main() {
    // If no args were passed, default will be last
    let arg = env::args().next().unwrap().to_string(); // first argument
    let command = env::args().last().unwrap().to_string();
    if command == arg {
        println!("ERROR\n\tRequires the command as an argument");
        exit(1)
    }
    println!("{}", command);
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .status()  // check for status
        .expect("Failed to execute command");
    if !output.success() {
        println!("ERROR\n\tCommand failed with an error code: {:?}", output.code());
    }
}
