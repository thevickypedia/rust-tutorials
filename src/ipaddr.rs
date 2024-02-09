use std::net::{SocketAddr, UdpSocket};

fn get_local_ip_address() -> Result<String, std::io::Error> {
    // todo: get public IP using a flag and include parser
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr: SocketAddr = socket.local_addr()?;
    let ip_address = local_addr.ip().to_string();
    Ok(ip_address)
}

fn main() {
    match get_local_ip_address() {
        Ok(ip_address) => println!("Local IP address: {}", ip_address),
        Err(error) => eprintln!("Error: {}", error),
    }
}
