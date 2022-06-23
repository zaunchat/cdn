use std::env;

lazy_static! {
    pub static ref PORT: String = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    pub static ref S3_KEY: String = env::var("S3_KEY").expect("S3_KEY is required");
    pub static ref S3_SECRET: String = env::var("S3_SECRET").expect("S3_SECRET is required");
    pub static ref S3_ENDPOINT: String = env::var("S3_ENDPOINT").expect("S3_ENDPOINT is required");
    pub static ref S3_REGION: String = env::var("S3_REGION").unwrap_or_default();
}
