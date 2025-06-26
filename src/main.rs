
use actix_web::{web, App, HttpServer};
use std::io::Result;
use dotenv::dotenv;
use sqlx::{Pool, MySql};

mod db;
mod routes;
mod handlers;
mod models;

use db::sql_connection::start_connection;
pub struct AppState {
    sql_client: Pool<MySql>
}

#[allow(unused)]
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _pool = start_connection().await;
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                sql_client: _pool.clone()
            })
            )
            .configure(routes::users::users_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}