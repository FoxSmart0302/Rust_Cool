use crate::error::XResult;
use crate::product::Product;
use crate::source::{Sink, Source};
use async_compression::tokio::bufread::GzipDecoder;
use rs_models::env;
use s3::creds::Credentials;
use s3::Bucket;
use tokio::io::AsyncReadExt;
use tracing::error;

pub struct RemoteGzippedJsonStore {
    path: String,
}

impl RemoteGzippedJsonStore {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl RemoteGzippedJsonStore {
    fn get_bucket(&self) -> XResult<Bucket> {
        let credentials = Credentials::new(
            Some(env("S3_ACCESS_KEY").as_str()),
            Some(env("S3_SECRET_KEY").as_str()),
            None,
            None,
            None,
        )?;
        let region = env("S3_REGION").parse()?;
        let bucket = Bucket::new(&env("S3_BUCKET"), region, credentials)?;
        Ok(bucket)
    }

    #[allow(dead_code)]
    async fn delete(&self, path: &str) -> XResult<()> {
        let bucket = self.get_bucket()?;
        bucket.delete_object(path).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Source for RemoteGzippedJsonStore {
    async fn read(&self) -> XResult<Vec<Product>> {
        let bucket = self.get_bucket()?;

        let r = bucket.get_object(&self.path).await?;

        let mut d = GzipDecoder::new(r.as_slice());
        let mut buf = Vec::new();
        d.read_to_end(&mut buf).await?;

        let s = String::from_utf8(buf)?;

        // Split it into lines
        let mut out = vec![];
        for line in s.split('\n') {
            if line.trim().is_empty() {
                continue;
            }

            // And parse each line
            match serde_json::from_str::<Product>(line) {
                Ok(p) => out.push(p),
                Err(e) => {
                    error!(error = e.to_string(), line, "error deserializing line");
                    return Err(e.into());
                }
            }
        }

        Ok(out)
    }
}

#[async_trait::async_trait]
impl Sink for RemoteGzippedJsonStore {
    async fn write(&self, products: &[Product]) -> XResult<()> {
        let mut content = Vec::new();

        let mut gz = flate2::write::GzEncoder::new(&mut content, flate2::Compression::default());
        for p in products {
            jsonl::write(&mut gz, &p)?;
        }
        gz.finish()?;

        let bucket = self.get_bucket()?;

        bucket.put_object(&self.path, &content).await?;

        Ok(())
    }
}

#[cfg(test)]
mod remote_gzipped_json_test {
    use crate::error::XResult;
    use crate::product::Product;
    use crate::source::remote_gzipped_json_store::RemoteGzippedJsonStore;
    use crate::source::{Sink, Source};
    use dotenvy::dotenv;
    use uuid::Uuid;

    #[tokio::test]
    #[ignore]
    async fn it_works() -> XResult<()> {
        dotenv()?;
        let s = RemoteGzippedJsonStore::new("results/70.json.gz".to_string());
        let r = s.read().await?;
        assert_eq!(508, r.len());
        Ok(())
    }

    #[tokio::test]
    async fn it_can_write_and_read() -> XResult<()> {
        dotenv()?;

        let products = Vec::from_iter((0..100).map(|_| Product::fake_no_extra()));

        let path = format!("tests/{}", Uuid::new_v4().to_string());
        let s = RemoteGzippedJsonStore::new(path.clone());

        s.write(&products).await?;
        let res = s.read().await?;
        s.delete(&path).await?;

        assert_eq!(products, res);

        Ok(())
    }
}
