use argon2::Argon2;
use axum::{
    extract::{Json, State}

    ,
    http::StatusCode,
};
use jsonwebtoken::{encode, EncodingKey};
use jsonwebtoken::Header;
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Related};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    sub:i32, // subject: user id
    email:String,
    role_id: i32,
    exp:usize // expiration timestamp
}

use crate::AppState;
#[derive(Serialize)]
pub struct LoginResponse{
    id: i32,
    name: String,
    email: String,
    role_id:i32,
    role_name:String,
    token:String,
}
use crate::entities::{user, role};

pub async fn login_handler(State(state): State<AppState>, Json(payload):Json<LoginRequest>)
    -> Result<Json<LoginResponse>,(StatusCode, String)>{
    // 1. Find user by email
    let maybe_user = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(&state.db)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error {e}"))
        })?;

    let user = match maybe_user{
        Some(u)=>u,
        None=>{
            return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".into()))
        }
    };

    // 2. Fetch the role associated with the user.
    let maybe_role = user::Entity::find_related()
        .filter(role::Column::Id.eq(user.role_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Role lookup error: {e}"),
                )
        })?;
    let role = match maybe_role{
        Some(r)=>r,
        None=>{
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Role not found".into()))
        }
    };

    //2. Verify password using Argon2
    //
    // IMPORTANT:
    // `user.password` must be an Argon2 hash string, like:
    // "$argon2id$v=19$m=65536,t=3,p=1$...."
    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR,
        "Invalid stored password hash".into(),
        )
    })?;
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".into()));
    }

    // 3. Create JWT
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT_SECRET missing".to_string(),
            )
        })?;


    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims{
        sub: user.id,
        email:user.email.clone(),
        role_id: user.role_id,
        exp:expiration,
    };
    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Token creation error:{e}")
            )
        })?;

    // 4. Return LoginResponse with JWT included
    let response = LoginResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        role_id: user.role_id,
        role_name:role.name,
        token,
    };

    Ok(Json(response))




}