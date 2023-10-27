use std::net::SocketAddr;

fn main() {
    println!("Hello, world!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("{}", addr.to_string());
}
