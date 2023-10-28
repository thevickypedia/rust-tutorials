extern crate reqwest;

use std::env;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let arguments: Vec<String> = env::args().collect();
    let token = arguments.last().unwrap().to_string();
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://jarvis.vigneshrao.com/secure-send")
        .header("access-token", token)
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
