use bcrypt::{DEFAULT_COST, hash, verify};

pub fn encode(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, 10)
}

pub fn verify_password(
    plain_password: &str,
    stored_hash: &str,
) -> Result<bool, bcrypt::BcryptError> {
    verify(plain_password, stored_hash)
}
