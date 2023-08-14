use crate::error::XResult;
use crate::simple_repo;
use crate::user::User;
use sqlx::{Pool, Postgres};

simple_repo!(UserRepo);

impl UserRepo {
    pub async fn find(&self, id: i64) -> XResult<User> {
        let row = sqlx::query_as::<_, User>("SELECT * from users where id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }
}
