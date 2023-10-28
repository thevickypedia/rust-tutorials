use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn write_file(filename: &str, data: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("ERROR:\n\tNo filename received.\n\tUsage: {} <filename>", arguments[0]);
        exit(1);
    }
    let filename = &arguments[1];
    let file_path = Path::new(filename);
    if file_path.exists() {
        println!("ERROR:\n\t{} already exists", filename);
        exit(1);
    }
    match write_file(filename, "hello-world") {
        Ok(_) => println!("Data written to {}", filename),
        Err(err) => eprintln!("Error: {}", err),
    }
}
