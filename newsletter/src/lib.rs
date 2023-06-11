use actix_web::HttpResponse;
use actix_web::{dev::Server, middleware, App, HttpServer};
use actix_web::{post, web};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub mod cfg;

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
            .service(subscribe)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("email: {}, name: {}", form.email, form.name);
    HttpResponse::Ok().finish()
}
