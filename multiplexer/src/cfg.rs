#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub app_port: u16,
}

pub fn get_config() -> Result<AppSettings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            &format!("{}/cfg.toml", env!("CARGO_MANIFEST_DIR")),
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize::<AppSettings>()
}
