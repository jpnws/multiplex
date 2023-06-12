use env_logger::Env;
use multiplexer::cfg::AppSettings;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let cfg = AppSettings::new().expect("Failed to read configuration.");
    let newsletter_db_settings =
        newsletter::cfg::DatabaseSettings::new().expect("Failed to get config.");
    let db_pool = PgPool::connect(&newsletter_db_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", cfg.app_port);
    let listener = TcpListener::bind(address)?;
    multiplexer::run(listener, db_pool)?.await
}
