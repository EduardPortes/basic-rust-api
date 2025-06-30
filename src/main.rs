use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::{MySql, Pool};
use std::io::Result;

mod auth;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use db::sql_connection::start_connection;
pub struct AppState {
    sql_client: Pool<MySql>,
}

#[allow(unused)]
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _pool = start_connection().await;
    let auth_middleware = HttpAuthentication::bearer(utils::token_utils::validate_token);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                sql_client: _pool.clone(),
            }))
            .configure(routes::auth::auth_routes)
            .service(
                web::scope("/api")
                    .wrap(auth_middleware.clone())
                    .configure(routes::users::users_routes),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
