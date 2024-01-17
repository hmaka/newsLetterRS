#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DataBaseSettings,
    pub application_port: u16,
}
impl DataBaseSettings {
    pub fn connection_string(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        );
    }

    pub fn connection_string_without_db(&self) -> String {
        return format! {
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        };
    }
}
impl TryFrom<config::Config> for Settings {
    type Error = config::ConfigError;
    fn try_from(value: config::Config) -> Result<Self, Self::Error> {
        Ok(Settings {
            database: value.get("database")?,
            application_port: value.get("application_port")?,
        })
    }
}

#[derive(serde::Deserialize)]
pub struct DataBaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings_builder = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;
    return settings_builder.try_into();
}
