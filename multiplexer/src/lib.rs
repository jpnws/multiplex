use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

async fn greet(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello {}!", &name))
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/health_check", web::get().to(health_check))
            .route("/greeting/{name}", web::get().to(greet))
            .service(bbs::get_board_by_id)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    Ok(server)
}
