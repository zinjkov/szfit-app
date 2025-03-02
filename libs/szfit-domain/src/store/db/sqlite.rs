// use sqlx::sqlite::{SqlitePoolOptions, SqliteRow};
// use sqlx::{Pool, Sqlite};
//
// pub type Db = Pool<Sqlite>;
// pub type DbType = Sqlite;
// pub type DbRow = SqliteRow;
//
// pub async fn new_db_pool() -> sqlx::Result<Db> {
//     let max_connections = 5; //if cfg!(test) { 1 } else { 5 };
//     println!("Db Url: {}", szfit_utils::app_config().DB_URL);
//     SqlitePoolOptions::new()
//         .max_connections(max_connections)
//         .connect(&szfit_utils::app_config().DB_URL)
//         .await
// }
