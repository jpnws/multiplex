use actix_web::{dev::Server, middleware, post, web, App, HttpResponse, HttpServer};
use cfg::DatabaseSettings;
use chrono::Utc;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use uuid::Uuid;

pub mod cfg;

pub struct TestApp {
    pub address: SocketAddrV4,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database.
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to create dabase.");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, &config.database.name))
        .await
        .expect("Failed to create database.");

    // Migrate database.
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");
    connection_pool
}

pub async fn spawn_app() -> TestApp {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(ip_addr, 0);
    let listener = TcpListener::bind(socket_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let mut cfg = cfg::DatabaseSettings::new().expect("Failed to get config.");
    cfg.database.name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&cfg).await;
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
