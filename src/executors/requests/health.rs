extern crate reqwest;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let resp = reqwest::blocking::get("https://jarvis.vigneshrao.com/health");
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
