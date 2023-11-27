pub struct AppState {
    pub db: PgPool,
    pub config: EnvConfig,
}

pub struct EnvConfig {
    pub jwt_secret: String,
    pub database_url: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not found");

        Self {
            jwt_secret,
            database_url,
        }
    }
}
