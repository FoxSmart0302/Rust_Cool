#![cfg(test)]
use crate::error::XResult;
use crate::product::Product;
use crate::source::{Sink, Source};
use async_compression::tokio::bufread::GzipDecoder;
use tokio::io::AsyncReadExt;
use tracing::error;

pub struct LocalGzippedJsonStore {
    path: String,
}

impl LocalGzippedJsonStore {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait::async_trait]
impl Source for LocalGzippedJsonStore {
    async fn read(&self) -> XResult<Vec<Product>> {
        let data = tokio::fs::read(&self.path).await?;

        let mut d = GzipDecoder::new(data.as_slice());
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
impl Sink for LocalGzippedJsonStore {
    async fn write(&self, _products: &[Product]) -> XResult<()> {
        todo!()
        // let mut content = Vec::new();
        //
        // let mut gz = flate2::write::GzEncoder::new(&mut content, flate2::Compression::default());
        // for p in products {
        //     jsonl::write(&mut gz, &p)?;
        // }
        // gz.finish()?;
        //
        // let bucket = self.get_bucket()?;
        //
        // bucket.put_object(&self.path, &content).await?;
        //
        // Ok(())
    }
}

#[cfg(test)]
mod local_gzipped_json_test {
    use crate::error::XResult;
    use crate::source::local_gzipped_json_store::LocalGzippedJsonStore;
    use crate::source::Source;
    use dotenvy::dotenv;
    use env_logger::Env;

    #[tokio::test]
    #[ignore]
    async fn it_can_read() -> XResult<()> {
        dotenv()?;

        let env_logger = Env::default()
            .filter_or("MY_LOG_LEVEL", "trace")
            .write_style_or("MY_LOG_STYLE", "always");
        env_logger::init_from_env(env_logger);

        let s = LocalGzippedJsonStore::new("../../32574.json.gz".to_string());
        let r = s.read().await?;
        assert_eq!(7842, r.len());
        Ok(())
    }
}
