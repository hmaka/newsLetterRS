#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DataBaseSettings,
    pub application_port: u16,
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

//impl From<config::Config> for Settings {
//    fn from(config: config::Config) -> Self {
//        // You will need to extract the relevant fields from the config object and use them to construct a Settings object.
//        // This is just a placeholder. Replace it with your actual logic.
//        Settings {
//            database: config.get("database"),
//            application_port: config.get("application_port"),
//        }
//    }
//}
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
