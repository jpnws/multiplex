use actix_web::{dev::Server, middleware, post, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use sqlx::PgPool;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use uuid::Uuid;

pub mod cfg;

pub struct TestApp {
    pub address: SocketAddrV4,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let cfg = cfg::Settings::new().expect("Failed to get config.");
    let connection_pool = PgPool::connect(&cfg.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let server = run(listener, connection_pool.clone())
        .await
        .expect("Failed to bind address.");
    tokio::spawn(server);
    TestApp {
        address: SocketAddrV4::new(ip_addr, port),
        db_pool: connection_pool,
    }
}

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(subscribe)
            .app_data(db_pool.clone())
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
pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
