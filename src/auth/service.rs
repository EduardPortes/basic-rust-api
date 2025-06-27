use super::models::LoginRequest;
use crate::AppState;
use crate::db::user_db;
use crate::utils::{crypto_utils, token_utils};
use actix_web::HttpResponse;
use actix_web::web::Data;

pub struct AuthService;

impl AuthService {
    pub async fn authenticate(
        app_state: Data<AppState>,
        form: LoginRequest,
    ) -> Result<String, AuthError> {
        let email = form.email.clone();
        let password = form.password.clone();

        if (form.email.is_empty() || form.password.is_empty()) {
            return Err(AuthError::InvalidCredentials);
        }

        let user = user_db::get_by_email(app_state, email)
            .await
            .map_err(|_| AuthError::DatabaseError)?;

        let is_same = crypto_utils::verify_password(&password, &user.password)
            .expect("Failed to verify password");

        if (!is_same) {
            return Err(AuthError::InvalidCredentials);
        }

        let token = token_utils::create_token(user.id).expect("Failed to create token");

        Ok(token)
    }
}

pub enum AuthError {
    InvalidCredentials,
    DatabaseError,
    TokenCreationError,
}
