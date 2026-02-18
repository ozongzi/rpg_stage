use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub deepseek_token: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::Environment::default())
        .build()?;

    settings.try_deserialize()
}
