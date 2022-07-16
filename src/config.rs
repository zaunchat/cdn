use std::env;

macro_rules! get {
    ($key:expr) => {{
        env::var($key).expect(&format!("{} is required", $key))
    }};
    ($key:expr, $default: expr) => {{
        env::var($key).unwrap_or($default.to_string())
    }};
}

lazy_static! {
    pub static ref PORT: String = get!("PORT", "8080");
    pub static ref ORIGIN: String = get!("ORIGIN", "*");
    pub static ref DATABASE_URI: String = get!(
        "DATABASE_URI",
        "postgres://postgres:postgres@localhost:5432"
    );
    pub static ref DATABASE_POOL_SIZE: u32 = get!("DATABASE_POOL_SIZE", "10").parse().unwrap();
    pub static ref S3_KEY: String = get!("S3_KEY", "s3-storage");
    pub static ref S3_SECRET: String = get!("S3_SECRET", "passw0rd");
    pub static ref S3_ENDPOINT: String = get!("S3_ENDPOINT", "http://127.0.0.1:10000");
    pub static ref S3_REGION: String = get!("S3_REGION", "");
}
