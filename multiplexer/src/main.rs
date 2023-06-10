use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    multiplexer::run(listener)?.await
}
