use sea_orm::{DatabaseConnection, RelationTrait};
use sea_orm::QuerySelect;
use argon2::Argon2;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use jsonwebtoken::{encode, EncodingKey};
use jsonwebtoken::Header;
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter,};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use opentelemetry::trace::TraceContextExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use crate::handlers::structs::Claims;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
/// JWT Claims
/// This struct represents the information in the JWT token, which after encoding,
/// becomes the token field in the LoginResponse struct.

#[derive(Serialize, Deserialize,)]
#[serde(rename_all = "lowercase")]
pub enum MenuState {
    Enabled,
}

pub type MenuActivationMap = HashMap<String, MenuState>;



use crate::AppState;
#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    user_id: i32,
    name: String,
    email: String,
    role_id:i32,
    role_name:String,
    pub token:String,
    menu_activation_map:MenuActivationMap,
}
use crate::entities::{user, permission, data_object, role_permission};
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload):Json<LoginRequest>,
) -> Result<Json<LoginResponse>,(StatusCode, String)>{
    let context = tracing::Span::current().context();
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid(){
        tracing::info!("SUCCESS: Rust is linked to Trace ID:{}", span_context.trace_id());
    }else{
        tracing::warn!("FAILURE: No valid Trace ID found");
    }


    // 1. Find the user by email
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
    let maybe_role= crate::entities::role::Entity::find_by_id(user.role_id)
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

    let menu_activation_map = build_menu_activation_map(& state.db, role.id).await.unwrap();

    // 4. Return LoginResponse with JWT included
    let response = LoginResponse {
        user_id: user.id,
        name: user.name,
        email: user.email,
        role_id: user.role_id,
        role_name:role.name,
        token,
        menu_activation_map,
    };

    Ok(Json(response))

}

use crate::entities::sea_orm_active_enums::PermissionActionEnum;
async fn build_menu_activation_map(db: &DatabaseConnection, role_id:i32)->Result<MenuActivationMap, DbErr>{
    let mut menu_activation_map:MenuActivationMap = MenuActivationMap::new();
    let readable_data_objects:Vec<String>= role_permission::Entity::find()
        .join( sea_orm::JoinType::InnerJoin, role_permission::Relation::Permission.def(),)
        .join( sea_orm::JoinType::InnerJoin, permission::Relation::DataObject.def(),)
        .filter(permission::Column::Action.eq(PermissionActionEnum::Read))
        .filter(role_permission::Column::RoleId.eq(role_id))
        .select_only()
        .column(data_object::Column::Name)
        .distinct()
        .into_tuple::<(String,)>()
        .all(db)
        .await?
        .into_iter()
        .map(|(name,) | name)
        .collect();


    for data_object in readable_data_objects{
        menu_activation_map.insert(data_object, MenuState::Enabled);
    }


    Ok(menu_activation_map)

}