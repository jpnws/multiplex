pub mod configuration;

use actix_web::{dev::Server, get, middleware, App, HttpResponse, HttpServer};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub fn spawn_app() -> SocketAddrV4 {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    SocketAddrV4::new(ip_addr, port)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(check_health)
            .service(newsletter::subscribe)
            .service(bbs::get_board_by_id)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/check_health")]
async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
