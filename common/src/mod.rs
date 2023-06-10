use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub fn spawn_app() -> SocketAddrV4 {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = multiplexer::run(listener).expect("Failed to bind address.");
    // Launch the server as a background task.
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence no variable binding.
    tokio::spawn(server);
    SocketAddrV4::new(ip_addr, port)
}
