use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(bbs::get_board_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
