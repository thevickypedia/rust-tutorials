extern crate reqwest;

use std::env;
use std::process::exit;

mod environ;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let token = environ::get("token");
    if token.is_empty() {
        println!("Requires auth token!!");
        exit(1)
    }
    let command = env::args().last().unwrap().to_string();
    let json_data = format!(r#"{{"command": "{}"}}"#, command);
    println!("{}", json_data);
    let mut auth_header = String::new();
    auth_header.push_str("Bearer ");
    auth_header.push_str(token.as_str());
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://jarvis.vigneshrao.com/offline-communicator")
        .header("Authorization", auth_header)
        .body(json_data)
        .send();
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
