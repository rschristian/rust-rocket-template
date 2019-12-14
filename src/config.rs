use chrono::{DateTime, Duration, Utc};

pub const SECRET: &str = "secret123";
pub const TOKEN_PREFIX: &str = "Bearer ";

pub fn token_expire_time() -> DateTime<Utc> {
    Utc::now() + Duration::days(60)
}
