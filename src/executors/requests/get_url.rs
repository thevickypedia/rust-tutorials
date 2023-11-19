extern crate reqwest;

use std::env;
use std::process::exit;

mod parser;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let arguments: Vec<String> = env::args().collect();
    let request_url = &arguments.last().unwrap().to_string();
    if parser::parse_url(request_url) {
        println!("URL OK: {}", request_url);
    } else {
        exit(1)
    }
    let resp = reqwest::blocking::get(request_url);
    match resp {
        Ok(ok) => {
            let status = ok.status();
            if status.as_u16() < 400 {
                println!("{:?}", ok.text());
            } else {
                println!("{}", ok.status())
            }
        }
        Err(err) => {
            println!("ERROR!!!");
            println!("{}", err.to_string())
        }
    }
}
