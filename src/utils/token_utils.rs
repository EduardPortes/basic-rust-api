use crate::auth::models::Claims;
use actix_web::{Error, HttpMessage, dev::ServiceRequest, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken;
use jsonwebtoken::{DecodingKey, Validation, decode};

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap()
}

pub fn create_token(user_id: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id.to_string(), 3600);

    let header = jsonwebtoken::Header::default();
    let secret = get_secret();
    jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
}

pub async fn validate_token(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::default(),
    ) {
        Ok(token_access) => {
            req.extensions_mut().insert(token_access.claims);
            Ok(req)
        }
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token");
            Err((err, req))
        }
    }
}
