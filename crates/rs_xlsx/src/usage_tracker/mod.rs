use crate::error::XResult;
use crate::usage_tracker::lock::Lock;
use chrono::Utc;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

mod lock;
mod lua_scripts;

const TTL: i64 = 60 * 60 * 3; // 3 hours

#[async_trait::async_trait]
pub trait UsageTracker {
    /// Gets a list of all tables that are loaded for the given account_id.
    async fn tables(&self, account_id: i64) -> XResult<Vec<String>>;

    /// Try to acquire a lock on the given table.
    ///
    /// Returns true if the lock was acquired,
    /// false if the lock already exists.
    async fn lock(&self, table_name: &str) -> XResult<bool>;

    /// Try to acquire a lock on the given table.
    ///
    /// Will retry until the lock is acquired.
    async fn wait_for_lock(&self, table_name: &str) -> XResult<()>;

    /// Try to release the lock on the given table.
    ///
    /// Returns true if the lock was released,
    /// Returns false if the lock does not exist
    async fn release(&self, table_name: &str) -> XResult<bool>;

    /// Sets the dirty flag for the given table name
    async fn set_dirty(&self, table_name: &str, dirty: bool) -> XResult<()>;

    async fn is_dirty(&self, table_name: &str) -> XResult<bool>;

    async fn remove(&self, table_name: &str, account_id: i64) -> XResult<()>;

    /// Touches the given table, updating its last access time. The last
    /// access time is used to determine when to delete the table.
    async fn touch(&self, table_name: &str, account_id: i64) -> XResult<()>;

    /// Returns a list of _product_ tables only! that are due for expiration.
    /// It is assumed that the errors tables will be expired at the same time.
    async fn expired(&self) -> XResult<Vec<String>>;
}

/// UsageTracker keeps track of product and error tables that have been created.
///
/// It holds:
///   - the last touch time for each table
///   - a dirty flag for each table
///
/// We need to keep track of these since they are ephemeral and they will be
/// deleted after a certain amount of time.
///
/// It holds 3 keys:
///     1. scan_tables: a hash, keyed by table names, of the table's last touch time
///     2. scan_tables_dirty: a hash, keyed by table names, of whether the table is dirty
///     3. account_tables_<account_id>: a set of product table names that exist and are tied to the given account_id

#[derive(Clone)]
pub struct RedisUsageTracker {
    redis: ConnectionManager,
    lock: Lock,
}

impl RedisUsageTracker {
    pub fn new(redis: ConnectionManager) -> Self {
        Self {
            lock: Lock::new(redis.clone()),
            redis,
        }
    }
}

#[async_trait::async_trait]
impl UsageTracker for RedisUsageTracker {
    /// Gets a list of all tables that are loaded for the given account_id.
    async fn tables(&self, account_id: i64) -> XResult<Vec<String>> {
        let mut redis = self.redis.clone();

        let tables: Vec<String> = redis
            .smembers(format!("account_tables_{}", account_id))
            .await?;

        Ok(tables)
    }

    /// Try to acquire a lock on the given table.
    ///
    /// Returns true if the lock was acquired,
    /// false if the lock already exists.
    async fn lock(&self, table_name: &str) -> XResult<bool> {
        self.lock.acquire(table_name, None).await
    }

    async fn wait_for_lock(&self, table_name: &str) -> XResult<()> {
        let sleep = tokio::time::Duration::from_millis(100);
        loop {
            match self.lock.acquire(table_name, None).await? {
                true => return Ok(()),
                false => {
                    tokio::time::sleep(sleep).await;
                }
            }
        }
    }

    /// Try to release the lock on the given table.
    ///
    /// Returns true if the lock was released,
    /// Returns false if the lock does not exist
    async fn release(&self, table_name: &str) -> XResult<bool> {
        self.lock.release(table_name).await
    }

    /// Sets the dirty flag for the given table name
    async fn set_dirty(&self, table_name: &str, dirty: bool) -> XResult<()> {
        let mut redis = self.redis.clone();

        if dirty {
            redis.hset("scan_tables_dirty", table_name, "1").await?;
        } else {
            redis.hdel("scan_tables_dirty", table_name).await?;
        }

        Ok(())
    }

    async fn is_dirty(&self, table_name: &str) -> XResult<bool> {
        let mut redis = self.redis.clone();

        let v: Option<String> = redis.hget("scan_tables_dirty", table_name).await?;

        if let Some(v) = v {
            Ok(v == "1")
        } else {
            Ok(false)
        }
    }

    async fn remove(&self, table_name: &str, account_id: i64) -> XResult<()> {
        let mut redis = self.redis.clone();

        redis.hdel("scan_tables", table_name).await?;
        redis
            .srem(format!("account_tables_{}", account_id), table_name)
            .await?;

        self.set_dirty(table_name, false).await
    }

    /// Touches the given table, updating its last access time. The last
    /// access time is used to determine when to delete the table.
    async fn touch(&self, table_name: &str, account_id: i64) -> XResult<()> {
        let mut redis = self.redis.clone();

        let now = Utc::now().timestamp();
        redis.hset("scan_tables", table_name, now).await?;
        redis
            .sadd(format!("account_tables_{}", account_id), table_name)
            .await?;

        Ok(())
    }

    /// Returns a list of _product_ tables only! that are due for expiration.
    /// It is assumed that the errors tables will be expired at the same time.
    async fn expired(&self) -> XResult<Vec<String>> {
        let mut redis = self.redis.clone();

        let now = Utc::now().timestamp();

        // Returns a list of tuples for each table
        // First item will be the table name, second will be the last touch time, as a Unix timestamp
        let tables: Vec<(String, i64)> = redis.hgetall("scan_tables").await?;

        // Filter out tables that have not expired, returning the table name
        let is_expired = |(name, ts)| {
            if ts + TTL < now {
                Some(name)
            } else {
                None
            }
        };

        Ok(tables.into_iter().filter_map(is_expired).collect())
    }
}

#[cfg(test)]
mod usage_tracker_tests {
    use crate::error::XResult;
    use crate::usage_tracker::{RedisUsageTracker, UsageTracker, TTL};
    use chrono::Utc;
    use redis::AsyncCommands;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_works() -> XResult<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let c = client.get_tokio_connection_manager().await?;
        let tracker = RedisUsageTracker::new(c);

        let table_name = Uuid::new_v4().to_string();

        // From a fresh touch, the table should not be expired
        tracker.touch(&table_name, 1).await?;
        let r = tracker.expired().await?;
        assert_eq!(false, r.contains(&table_name));

        // Set the last touch time to be older than TTL
        let mut redis = tracker.redis.clone();
        let now = Utc::now().timestamp();
        redis
            .hset("scan_tables", &table_name, now - TTL - 100)
            .await?;

        // And now it should come back as expired
        let r = tracker.expired().await?;
        assert_eq!(true, r.contains(&table_name));

        Ok(())
    }
}
