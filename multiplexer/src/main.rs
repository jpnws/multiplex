use multiplexer::cfg::Settings;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cfg = Settings::new().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", cfg.app_port);
    let listener = TcpListener::bind(address)?;
    multiplexer::run(listener)?.await
}
