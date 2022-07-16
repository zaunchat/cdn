use crate::{config::*, utils::Tag};
use s3::{creds::Credentials, error::S3Error, Bucket, Region};

lazy_static! {
    static ref CREDENTIALS: Credentials =
        Credentials::from_env_specific(Some("S3_KEY"), Some("S3_SECRET"), None, None).unwrap();
    static ref REGION: Region = Region::Custom {
        region: S3_REGION.clone(),
        endpoint: S3_ENDPOINT.clone(),
    };
}

pub fn bucket(tag: &Tag) -> Bucket {
    Bucket::new(&tag.to_string(), REGION.clone(), CREDENTIALS.clone())
        .unwrap()
        .with_path_style()
}

pub async fn save(tag: &Tag, id: i64, content: &[u8]) -> Result<(), S3Error> {
    bucket(tag).put_object(id.to_string(), content).await?;
    Ok(())
}

pub async fn get(tag: &Tag, id: i64) -> Result<Vec<u8>, S3Error> {
    let (data, _) = bucket(tag).get_object(id.to_string()).await?;
    Ok(data)
}

pub async fn delete(tag: &Tag, id: i64) -> Result<(), S3Error> {
    bucket(tag).delete_object(id.to_string()).await?;
    Ok(())
}
