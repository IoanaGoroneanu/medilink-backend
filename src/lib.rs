use sqlx::PgPool;

pub mod config;
pub mod router;
pub mod db;
pub mod auth;

pub struct ApiContext {
    pub db: PgPool,
    pub jwt_secret: String,
}
