use config::Config;
use sqlx::PgPool;
use tokio::net::TcpListener;
use youtrack_backend::{
    config::{
        app::AppConfig,
        http_serve,
    },
    ApiContext,
};

pub const CONFIG_FILE: &str = "config/app.toml";

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug")
    ).init();
    let config = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE).required(false))
        .add_source(config::Environment::default())
        .build()
        .expect("Could not read config file")
        .try_deserialize::<AppConfig>()
        .expect("Could not deserialize config");

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .expect("Could not bind to port");

    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Could not connect to database");

    let ctx = ApiContext {
        db: pool,
        jwt_secret: config.jwt_secret,
    };

    log::info!("Server starting on port {}", config.port);
    let _server = http_serve(listener, ctx).await;
}
