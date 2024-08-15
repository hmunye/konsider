use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Deserialize)]
pub struct Config {
    #[clap(long, env)]
    pub postgres_user: String,

    #[clap(long, env)]
    pub postgres_password: String,

    #[clap(long, env)]
    pub postgres_db: String,

    #[clap(long, env)]
    pub postgres_port: u16,

    #[clap(long, env)]
    pub postgres_host: String,

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
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }
}
