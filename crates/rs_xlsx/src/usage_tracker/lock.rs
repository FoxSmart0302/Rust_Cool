use crate::error::XResult;
use crate::usage_tracker::lua_scripts::RELEASE;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::time::Duration;

const OWNER: usize = 1;

/// Lock models Laravel's cache lock, providing an atomic, distributed locking mechanism.
#[derive(Clone)]
pub struct Lock {
    redis: ConnectionManager,
}

impl Lock {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Attempt to acquire the lock on the given name. Returns true if the lock was acquired,
    /// false otherwise.
    ///
    /// Acquiring a lock will only ever return false if a lock already exists.
    pub async fn acquire(&self, name: &str, ttl: Option<Duration>) -> XResult<bool> {
        let mut redis = self.redis.clone();

        Ok(match ttl {
            Some(v) => redis.set_ex(name, OWNER, v.as_secs() as usize).await?,
            None => redis.set_nx(name, OWNER).await?,
        })
    }

    /// Release the lock on the given name. Returns true if the lock was released, false otherwise.
    ///
    /// The release will only ever return false if:
    ///   - the lock does not exist or
    ///   - we try and release a lock with an owner that
    ///     is different from the one it was set with.
    pub async fn release(&self, name: &str) -> XResult<bool> {
        let mut redis = self.redis.clone();

        let r = redis::Script::new(RELEASE)
            .key(name)
            .arg(OWNER)
            .invoke_async::<ConnectionManager, bool>(&mut redis)
            .await?;

        Ok(r)
    }

    /// Release the lock, regardless of the owner.
    ///
    /// Returns false if the lock does not exist.
    #[allow(dead_code)]
    pub async fn force_release(&self, name: &str) -> XResult<bool> {
        let mut redis = self.redis.clone();
        let cnt: usize = redis.del(name).await?;
        Ok(cnt > 0)
    }
}

#[cfg(test)]
mod lock_tests {
    use crate::error::XResult;
    use crate::usage_tracker::lock::Lock;
    use uuid::Uuid;

    #[tokio::test]
    async fn it_works() -> XResult<()> {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let c = client.get_tokio_connection_manager().await.unwrap();
        let l = Lock::new(c);

        // Generate a random key
        let key = Uuid::new_v4().to_string();

        // Clear the lock, if it exists
        l.force_release(&key).await?;

        // First acquire should succeed
        assert_eq!(true, l.acquire(&key, None).await?);

        // Second acquire should fail
        assert_eq!(false, l.acquire(&key, None).await?);

        // Release should succeed
        assert_eq!(true, l.release(&key).await?);

        // Force release should fail, since no lock exists
        assert_eq!(false, l.force_release(&key).await?);

        // Force release should succeed with an existing lock
        assert_eq!(true, l.acquire(&key, None).await?);
        assert_eq!(true, l.force_release(&key).await?);

        Ok(())
    }
}
