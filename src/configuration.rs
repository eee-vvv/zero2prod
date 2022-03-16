#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize config reader
    let mut settings = config::Config::default();

    // Add config values from a file named `configuration1`
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert config values into our Settings type
    settings.try_into()
}
