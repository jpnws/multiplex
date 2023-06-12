use actix_web::{
    dev::Server, get, middleware::Compress, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use sqlx::PgPool;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub mod cfg;

pub fn spawn_app() -> SocketAddrV4 {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run_for_tests(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    SocketAddrV4::new(ip_addr, port)
}

pub fn run_for_tests(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(check_health)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(check_health)
            .service(newsletter::subscribe)
            .service(bbs::get_board_by_id)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/check_health")]
async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
