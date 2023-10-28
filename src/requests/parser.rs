extern crate reqwest;

use reqwest::Url;

pub fn parse_url(request_url: &str) -> bool {
    if let Ok(url) = Url::parse(request_url) {
        let scheme = url.scheme();
        let host = url.host_str().unwrap();
        let domain = url.domain().unwrap();
        let path = url.path();
        println!("scheme: {}", scheme);
        if host == domain {
            println!("host: {}", host);
        } else {
            println!("host: {}", host);
            println!("domain: {}", domain);
        }
        println!("path: {}", path);
        return true;
    } else {
        println!("'{}' is not a URL", request_url);
        return false;
    }
}
