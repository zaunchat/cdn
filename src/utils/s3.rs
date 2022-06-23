use crate::config;
use s3::{creds::Credentials, error::S3Error, Bucket, Region};

lazy_static! {
    static ref CREDENTIALS: Credentials = Credentials {
        access_key: Some(config::S3_KEY.clone()),
        secret_key: Some(config::S3_SECRET.clone()),
        security_token: None,
        session_token: None,
    };
    static ref REGION: Region = Region::Custom {
        region: config::S3_REGION.clone(),
        endpoint: config::S3_ENDPOINT.clone(),
    };
}

pub fn bucket(tag: String) -> Bucket {
    Bucket::new(&tag, REGION.clone(), CREDENTIALS.clone()).unwrap()
}

pub async fn save(tag: String, id: u64, data: &[u8]) -> Result<(), S3Error> {
    bucket(tag).put_object(id.to_string(), data).await?;
    Ok(())
}

#[cfg(allow_dead)]
pub async fn get(tag: String, id: u64) -> Result<Vec<u8>, S3Error> {
    let (data, _) = bucket(tag).get_object(id.to_string()).await?;
    Ok(data)
}

#[cfg(allow_dead)]
pub async fn delete(tag: String, id: u64) -> Result<(), S3Error> {
    bucket(tag).delete_object(id.to_string()).await?;
    Ok(())
}
