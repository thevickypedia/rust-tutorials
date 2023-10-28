mod parser;

extern crate reqwest;

use std::env::args;
use std::process::exit;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let arguments: Vec<String> = args().collect();
    let request_url = &arguments.last().unwrap().to_string();
    if parser::parse_url(request_url) {
        println!("URL OK: {}", request_url);
    } else {
        exit(1)
    }
    let resp = reqwest::blocking::get(request_url);
    if resp.is_ok() {
        if let Ok(text) = resp.unwrap().text() {
            println!("{}", text.as_str());
        } else {
            println!("Failed to read response text");
        }
    } else if resp.is_err() {
        println!("{:?}", resp.err());
    } else {
        println!("{:?}", resp.unwrap().error_for_status());
    }
}
