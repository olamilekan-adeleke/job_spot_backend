use sqlx::PgPool;
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: EnvConfig,
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub jwt_secret: String,
    pub database_url: String,
    pub server_port: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not found");
        let server_port = env::var("PORT").expect("PORT is not found");

        Self {
            jwt_secret,
            database_url,
            server_port,
        }
    }
}
