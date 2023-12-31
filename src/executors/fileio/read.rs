use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = arguments.last().unwrap().to_string();
    let file_object = Path::new(&filename);
    if !file_object.exists() {
        println!("ERROR:\n\tFile '{}' doesn't exist", filename);
        exit(1)
    }
    if !file_object.is_file() {
        println!("ERROR:\n\t'{}' is not a file", filename);
        exit(1)
    }
    // Use clone since value of filename cannot be borrowed after move
    match read_file(filename.clone()) {
        Ok(contents) => {
            let liner = "-".repeat(80);
            println!("{}\n{}\n{}", liner, contents, liner);
        }
        Err(error) => {
            eprintln!("ERROR:\n\tFailed to read file '{}'\n\t{}", filename, error);
        }
    }
}
