use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres};
use std::env;

pub fn env(key: &'static str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

pub fn pg_opts(db_name: &str) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&env("DB_HOST"))
        .port(env("DB_PORT").parse::<u16>().unwrap())
        .username(&env("DB_USERNAME"))
        .password(&env("DB_PASSWORD"))
        .database(db_name)
        // Statement caching seems to cause some _big_ slowdowns.
        // https://stackoverflow.com/a/75146091
        .statement_cache_capacity(0)
        // Super important! The sqlx query logger is _terribly_ slow, slowing down
        // upserts by a factor of ~100x
        .disable_statement_logging()
        .to_owned()
}

pub async fn get_pool(db_name: &str) -> Result<Pool<Postgres>, sqlx::error::Error> {
    let options = pg_opts(db_name);

    PgPoolOptions::new()
        .max_connections(50)
        .test_before_acquire(false)
        .connect_with(options)
        .await
}
