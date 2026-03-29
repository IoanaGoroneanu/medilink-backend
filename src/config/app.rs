use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Parser, Debug)]
pub struct AppConfig {
    #[arg(short, long)]
    pub host: String,
    #[arg(short, long)]
    pub port: String,
    #[arg(short, long)]
    pub database_url: String,
    #[arg(short, long)]
    pub jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            host: "0.0.0.0".to_string(),
            port: "9091".to_string(),
            database_url: String::new(),
            jwt_secret: String::new(),
        }
    }
}
