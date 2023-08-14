use crate::error::XResult;
use crate::product::Product;

mod local_gzipped_json_store;
mod postgres_store;
mod remote_gzipped_json_store;

pub use postgres_store::PostgresStore;
pub use remote_gzipped_json_store::RemoteGzippedJsonStore;

#[async_trait::async_trait]
pub trait Source {
    async fn read(&self) -> XResult<Vec<Product>>;
}

#[async_trait::async_trait]
pub trait Sink {
    async fn write(&self, content: &[Product]) -> XResult<()>;
}
