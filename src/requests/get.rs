extern crate reqwest;

fn main() {
    // if not blocking, method should be awaited 'reqwest.await.is_ok'
    let resp = reqwest::blocking::get("https://vigneshrao.com");
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
