use sqlx::{Pool, MySql};
use sqlx::mysql::MySqlPoolOptions;

pub async fn start_connection() -> Pool<MySql>{
    let mysql_environment = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_environment)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./src/db/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate");

    pool
}