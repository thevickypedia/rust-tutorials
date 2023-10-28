extern crate reqwest;

use reqwest::Url;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let request_url = "https://vigneshrao.com";
    if let Ok(url) = Url::parse(request_url) {
        println!("URL OK");
        println!("{}", url.as_str());
    }
    let resp = reqwest::blocking::get(request_url);
    if resp.is_ok() {
        println!("success!");
        if let Ok(text) = resp.unwrap().text() {
            println!("{}", text.as_str());
        } else {
            println!("Failed to read response text");
        }
    } else if resp.is_err() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.err());
    }
}
