use serde::{Deserialize, Serialize};
use jsonwebtoken::{DecodingKey, Validation};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub:i32, // subject: user id
    pub email:String,
    pub role_id: i32,
    pub exp:usize // expiration timestamp
}

#[derive(Clone)]
pub struct JwtConfig {
    pub decoding_key: DecodingKey,
    pub validation: Validation,
}
#[derive(Clone)]
pub struct AuthUser {
    pub claims: Claims,
}
