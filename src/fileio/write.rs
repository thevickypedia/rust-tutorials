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
    if arguments.len() < 2 {
        println!("ERROR:\n\tNo filename received.\n\tUsage: {} <filename>", arguments[0]);
        exit(1);
    }
    let filename = &arguments[1];
    let file_path = Path::new(filename);
    if file_path.exists() {
        println!("ERROR:\n\t{} already exists", filename);
        exit(1);
    }
    if arguments.len() > 3 {
        println!("WARNING\n\tToo many arguments received\n\t{:?}\n", &arguments[2..]);
    }
    let mut data = String::new();
    if arguments.len() >= 3 {
        let args_iter = &arguments[2..];
        for (ind, val) in args_iter.iter().enumerate() {
            println!("Value: '{}' at index: {}", val, ind);
            data.push_str(&val);
            data.push_str("\n")
        }
    } else {
        println!("WARNING\n\tNo content received as *args, so defaulting content to 'hello-world'");
        data = "hello-world".to_string();
    }
    match write_file(filename, &data) {
        Ok(_) => println!("Data written to {}", filename),
        Err(err) => eprintln!("Error: {}", err),
    }
}
