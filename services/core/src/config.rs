use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("OLYMPUS_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("OLYMPUS_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse::<u16>()
            .map_err(|_| "OLYMPUS_PORT must be a valid port".to_string())?;
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL is required".to_string())?;
        Ok(Self { host, port, database_url })
    }
}
