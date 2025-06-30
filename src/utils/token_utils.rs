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
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "sub", "iat"]);
    validation.leeway = 60;
    validation.validate_exp = true;

    let token_result = decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(get_secret().as_ref()),
        &validation,
    );

    match token_result {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => {
            let err = ErrorUnauthorized("Invalid token");
            Err((err, req))
        }
    }
}
