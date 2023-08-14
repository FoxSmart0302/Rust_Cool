use crate::error::{XError, XResult};
use crate::new_key_cache::NewKeyCache;
use crate::results::{table_name, TableType};
use crate::source::{PostgresStore, RemoteGzippedJsonStore, Sink, Source};
use crate::usage_tracker::{RedisUsageTracker, UsageTracker};
use actix_web::HttpRequest;
use auth::Auth;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use rs_models::{env, Repo, Scan};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub db: Pool<Postgres>,
    pub repo: Repo,
    pub redis: ConnectionManager,
    pub usage_tracker: RedisUsageTracker,
    pub auth: Auth,
    pub key_cache: Arc<NewKeyCache>,
}

impl State {
    pub async fn new(db: Pool<Postgres>, redis: ConnectionManager) -> XResult<Self> {
        let repo = Repo::new(db.clone());

        Ok(Self {
            repo: repo.clone(),
            db,
            usage_tracker: RedisUsageTracker::new(redis.clone()),
            redis,
            auth: Auth::from_tinker(&env("LARAVEL_PATH"))?,
            key_cache: Arc::new(NewKeyCache::from_repo(repo).await?),
        })
    }

    pub async fn auth_user(&self, r: &HttpRequest) -> XResult<i64> {
        let cookie = match r.cookie("rocket_source_session") {
            Some(cookie) => cookie,
            None => return Err(XError::NoAuthCookie),
        };

        let session_id = self.auth.get_session_id(cookie.value())?;
        let redis_key = self.auth.get_redis_key(&session_id)?;

        let mut redis = self.redis.clone();
        let php_serialized_data: String = redis.get(redis_key).await?;

        let user_id = self.auth.get_user_id(&php_serialized_data)?;

        Ok(user_id)
    }

    pub async fn save_if_dirty(&self, scan: &Scan) -> XResult<()> {
        // If it's not dirty, then nothing to do
        let products_table = table_name(scan.id, TableType::Products);
        if !self.usage_tracker.is_dirty(&products_table).await? {
            return Ok(());
        }

        // Load all the products
        let source = PostgresStore::new(self.db.clone(), scan.id);
        let products = source.read().await?;

        // And save those to a new gzipped json file
        let sink = RemoteGzippedJsonStore::new(format!("results/{}.json.gz", scan.id));
        sink.write(&products).await?;

        self.usage_tracker.set_dirty(&products_table, false).await?;

        Ok(())
    }
}
