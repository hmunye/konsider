use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub jwt_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub db_port: u16,
    pub require_ssl: bool,
}

impl DatabaseConfig {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(&self.password)
            .port(self.db_port)
            .ssl_mode(ssl_mode)
            .database(&self.database)
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // Detect the running environment (default to `local` if not provided)
    let environment: Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("failed to parse ENVIRONMENT");

    let env_filename = format!("{}.toml", environment.as_str());

    config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join(env_filename),
        ))
        .build()?
        .try_deserialize()
}

// Possible runtime environments
#[derive(Debug)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`",
                other
            )),
        }
    }
}
