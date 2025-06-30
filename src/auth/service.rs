use super::models::LoginRequest;
use crate::db::user_db;
use crate::utils::{crypto_utils, token_utils};
use crate::AppState;
use actix_web::web::Data;

pub struct AuthService;

impl AuthService {
    pub async fn authenticate(
        app_state: Data<AppState>,
        form: LoginRequest,
    ) -> Result<String, AuthError> {
        let email = form.email.clone();
        let password = form.password.clone();

        if form.email.is_empty() || form.password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }
        let user_service = user_db::UserDb::new(app_state.clone());
        let user = user_service
            .get_by_email(email)
            .await
            .map_err(|_| AuthError::DatabaseError)?;

        let is_same = crypto_utils::verify_password(&password, &user.password)
            .expect("Failed to verify password");

        if !is_same {
            return Err(AuthError::InvalidCredentials);
        }

        let token = token_utils::create_token(user.id)
            .map_err(|_| AuthError::TokenCreationError)?;

        Ok(token)
    }
}

pub enum AuthError {
    InvalidCredentials,
    DatabaseError,
    TokenCreationError,
}
