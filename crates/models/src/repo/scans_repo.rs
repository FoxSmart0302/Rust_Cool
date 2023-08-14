use crate::error::XResult;
use crate::Scan;
use sqlx::{Pool, Postgres};

/// Generates a simple repo struct with a pool and it's new method.
#[macro_export]
macro_rules! simple_repo {
    // `()` indicates that the macro takes no argument.
    ($struct_name: ident) => {
        #[derive(Clone)]
        pub struct $struct_name {
            pool: Pool<Postgres>,
        }

        impl $struct_name {
            pub fn new(pool: Pool<Postgres>) -> Self {
                Self { pool }
            }
        }
    };
}

simple_repo!(ScanRepo);

impl ScanRepo {
    pub async fn find(&self, id: i64) -> XResult<Scan> {
        let row = sqlx::query_as::<_, Scan>("SELECT * from scans where id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }
}
