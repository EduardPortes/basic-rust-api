use crate::models::user::{RegisterUser, UpdateUser, User};
use crate::AppState;
use actix_web::web;
use sqlx::mysql::MySqlQueryResult;
use sqlx::MySqlPool;

pub struct UserDb {
    pool: MySqlPool,
}

impl UserDb {
    pub fn new(app_state: web::Data<AppState>) -> Self {
        UserDb {
            pool: app_state.sql_client.clone(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM user")
            .fetch_all(&self.pool)
            .await;

        result
    }

    pub async fn get_by_id(&self, id: u64) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await;
        match result {
            Ok(user) => Ok(user),
            Err(sqlx::Error::RowNotFound) => Err(sqlx::Error::RowNotFound),
            Err(e) => Err(e),
        }
    }

    pub async fn get_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM user WHERE email = ?", email)
            .fetch_one(&self.pool)
            .await;
        match result {
            Ok(user) => Ok(user),
            Err(sqlx::Error::RowNotFound) => Err(sqlx::Error::RowNotFound),
            Err(e) => Err(e),
        }
    }

    pub async fn create(&self, user: RegisterUser) -> Result<MySqlQueryResult, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO user (username, email, password) VALUES (?, ?, ?)",
            user.username,
            user.email,
            user.password
        )
        .execute(&self.pool)
        .await;

        result
    }

    pub async fn update(&self, id: u64, user: UpdateUser) -> Result<UpdateUser, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE user SET username = ?, email = ?, password = ? WHERE id = ?",
            user.username,
            user.email,
            user.password,
            id
        )
        .execute(&self.pool)
        .await;

        result.map(|_| user).map_err(|e| e.into())
    }
}
