use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;
pub type DbType = Postgres;
pub type DbRow = PgRow;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    let max_connections = 5; //if cfg!(test) { 1 } else { 5 };
    println!("Db Url: {}", szfit_utils::app_config().DB_URL);
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&szfit_utils::app_config().DB_URL)
        .await
}