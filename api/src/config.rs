use clap::Parser;
use secrecy::{ExposeSecret, Secret};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub postgres_user: String,

    #[clap(long, env)]
    pub postgres_password: Secret<String>,

    #[clap(long, env)]
    pub postgres_db: String,

    #[clap(long, env)]
    pub postgres_host: String,

    #[clap(long, env)]
    pub postgres_port: u16,

    #[clap(long, env)]
    pub redis_host: String,

    #[clap(long, env)]
    pub redis_port: u16,

    #[clap(long, env)]
    pub server_host: String,

    #[clap(long, env)]
    pub server_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}

impl Config {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password.expose_secret(),
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.postgres_user,
            self.postgres_password.expose_secret(),
            self.postgres_host,
            self.postgres_port,
        ))
    }

    pub fn redis_uri(&self) -> Secret<String> {
        Secret::new(format!("redis://{}:{}", self.redis_host, self.redis_port))
    }
}
// ---------------------------------------------------------------------------------------------------------------
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

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use 'local' or 'production'",
                other
            )),
        }
    }
}
