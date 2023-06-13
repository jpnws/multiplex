use actix_web::{
    dev::Server, middleware::Compress, middleware::Logger, post, web, App, HttpResponse, HttpServer,
};
use cfg::DatabaseSettings;
use chrono::Utc;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use tracing::Instrument;
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
        .expect("Failed to create database.");
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
            .wrap(Compress::default())
            .wrap(Logger::default())
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
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span!` is equivalent to `info!` macro.
    let request_span = tracing::info_span!(
        "Add a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    // Using `enter` in an async function is a recipe for disaster!
    // Do not do this in production.
    // See the following section on `Instrumenting Futures`.
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Save new subscriber details to the database",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

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
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // This error log falls outside of `query_span`.
            tracing::error!("[REQID={}] Failed to save subscriber: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
    // `_request_span_guard` is dropped at the nd of `subscribe`.
    // That's when we "exit" the span.
}
