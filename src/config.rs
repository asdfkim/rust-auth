pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            server_addr: std::env::var("SERVER_ADDR").expect("SERVER_ADDR not set"),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        }
    }
}
