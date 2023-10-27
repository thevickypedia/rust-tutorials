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
    let filename = arguments.last().unwrap().to_string();
    // Use clone since value of filename cannot be borrowed after move
    match read_file(filename.clone()) {
        Ok(contents) => {
            let liner = "-".repeat(80);
            println!("{}\n{}\n{}", liner, contents, liner);
        }
        Err(error) => {
            eprintln!("\nError reading file '{}'\n\t{}", filename, error);
        }
    }
}
