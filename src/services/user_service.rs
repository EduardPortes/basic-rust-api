use crate::db::user_db::UserDb;
use crate::models::user::{RegisterUser, UpdateUser, User};
use crate::utils::crypto_utils;
use crate::AppState;
use actix_web::web;
use sqlx::mysql::MySqlQueryResult;

pub struct UserService {
    user_db: UserDb,
}

impl UserService {
    pub fn new(app_state: web::Data<AppState>) -> Self {
        UserService {
            user_db: UserDb::new(app_state),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_db.get_all().await
    }

    pub async fn create(
        &self,
        user: web::Json<RegisterUser>,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let pass_encoded = crypto_utils::encode(&user.password).expect("Failed to encode password");
        let new_user = RegisterUser {
            username: user.username.clone(),
            email: user.email.clone(),
            password: pass_encoded,
        };
        self.user_db.create(new_user).await
    }

    pub async fn update(
        &self,
        id: u64,
        user: web::Json<UpdateUser>,
    ) -> Result<UpdateUser, sqlx::Error> {
        let saved_user = self.user_db.get_by_id(id).await?;
        let changed = crypto_utils::verify_password(&user.password, &saved_user.password)
            .expect("Failed to verify password");
        let mut password = user.password.clone();
        if changed {
            password = crypto_utils::encode(&user.password).expect("Failed to encode password");
        }

        let update_user = UpdateUser {
            username: user.username.clone(),
            email: user.email.clone(),
            password,
        };
        self.user_db.update(id, update_user).await
    }
}
