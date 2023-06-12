use config::{Config, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    pub fn new() -> Result<DatabaseSettings, config::ConfigError> {
        let cfg_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let cfg_path = cfg_path.join("cfg.toml");
        let cfg_path = cfg_path.to_str().expect("Failed to get config path.");
        let settings = Config::builder()
            .add_source(File::new(cfg_path, config::FileFormat::Toml))
            .build()?;
        settings.try_deserialize::<DatabaseSettings>()
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name,
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.database.username, self.database.password, self.database.host, self.database.port
        )
    }
}
