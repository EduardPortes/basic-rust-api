use crate::AppState;
use crate::models::user::{RegisterUser, UpdateUser, User};
use crate::utils::crypto_utils;
use actix_web::web;
use sqlx::mysql::MySqlQueryResult;

pub async fn get_all(app_state: web::Data<AppState>) -> Result<Vec<User>, sqlx::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM user")
        .fetch_all(&app_state.sql_client)
        .await;

    result
}

pub async fn get_by_email(
    app_state: web::Data<AppState>,
    email: String,
) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM user WHERE email = ?", email)
        .fetch_one(&app_state.sql_client)
        .await;
    match result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::RowNotFound) => Err(sqlx::Error::RowNotFound),
        Err(e) => Err(e),
    }
}

pub async fn create(
    app_state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    println!("{}", user.password);
    let pass_encoded = crypto_utils::encode(&user.password).expect("Failed to encode password");
    println!("{}", pass_encoded);
    let pool = app_state.sql_client.clone();
    let result = sqlx::query!(
        "INSERT INTO user (username, email, password) VALUES (?, ?, ?)",
        user.username,
        user.email,
        pass_encoded
    )
    .execute(&pool)
    .await;

    result
}

pub async fn update(
    app_state: web::Data<AppState>,
    id: u64,
    user: web::Json<UpdateUser>,
) -> Result<UpdateUser, sqlx::Error> {
    let pool = app_state.sql_client.clone();
    let saved_user = sqlx::query!("SELECT * FROM user WHERE id = ?", id)
        .fetch_one(&pool)
        .await;

    let changed = crypto_utils::verify_password(&user.password, &saved_user?.password)
        .expect("Failed to verify password");
    let mut password = user.password.clone();

    if changed {
        password = crypto_utils::encode(&user.password).expect("Failed to encode password");
    }
    let result = sqlx::query!(
        "UPDATE user SET username = ?, email = ?, password = ? WHERE id = ?",
        user.username,
        user.email,
        password,
        id
    )
    .execute(&pool)
    .await;

    result.map(|_| user.into_inner()).map_err(|e| e.into())
}
