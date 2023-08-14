use crate::error::XResult;
use crate::note::Note;
use crate::simple_repo;
use sqlx::{Pool, Postgres};

simple_repo!(NoteRepo);

impl NoteRepo {
    pub async fn all_for(&self, marketplace_id: i16, account_id: i64) -> XResult<Vec<Note>> {
        let rows = sqlx::query_as::<_, Note>(
            "SELECT * from notes where marketplace_id = $1 and account_id = $2",
        )
        .bind(marketplace_id)
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
