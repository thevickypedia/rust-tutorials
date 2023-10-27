use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() == 1 {
        println!("FileName is required!!");
        exit(1);
    }
    println!("{:?}", arguments);
    let filename = arguments.last().unwrap().to_string();
    match read_file(filename) {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }
}
