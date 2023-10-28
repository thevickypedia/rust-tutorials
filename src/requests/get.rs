extern crate reqwest;

use std::env::args;
use std::process::exit;
use reqwest::Url;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let arguments: Vec<String> = args().collect();
    let request_url = &arguments.last().unwrap().to_string();
    if let Ok(url) = Url::parse(request_url) {
        let url_str = url.as_str();
        let scheme = url.scheme();
        let host = url.host_str().unwrap();
        let domain = url.domain().unwrap();
        let path = url.path();
        println!("URL OK: {}", url_str);
        println!("scheme: {}", scheme);
        if host == domain {
            println!("host: {}", host);
        } else {
            println!("host: {}", host);
            println!("domain: {}", domain);
        }
        println!("path: {}", path);
    } else {
        println!("'{}' is not a URL", request_url);
        exit(1)
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
