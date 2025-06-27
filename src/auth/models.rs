use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String, validity_secs: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(validity_secs);
        Claims {
            sub,
            iat: iat.timestamp() as usize,
            exp: exp.timestamp() as usize,
        }
    }

    pub fn is_valid(&self) -> bool {
        let current_time = chrono::Utc::now().timestamp() as usize;
        current_time < self.exp
    }
}
