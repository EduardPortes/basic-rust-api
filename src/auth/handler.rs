use super::models::LoginRequest;
use super::service::{AuthError, AuthService};
use crate::AppState;
use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, post};

#[post("/oauth/token")]
pub async fn oauth_token(app_state: Data<AppState>, form: Form<LoginRequest>) -> HttpResponse {
    let result = AuthService::authenticate(app_state, form.into_inner()).await;

    match result {
        Ok(token) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "access_token": token,
                "token_type": "bearer",
                "expires_in": 3600,
            }));
        }
        Err(AuthError::InvalidCredentials) => HttpResponse::BadRequest().finish(),
        Err(AuthError::DatabaseError) => HttpResponse::InternalServerError().body("Database error"),
        Err(AuthError::TokenCreationError) => {
            HttpResponse::InternalServerError().body("Token creation error")
        }
    }
}
