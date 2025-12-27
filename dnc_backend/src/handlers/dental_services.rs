use axum::{extract::State, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ColumnTrait,  EntityTrait, QueryFilter};
use crate::entities::dental_service;
pub async fn get_all_dental_services(State(state): State<AppState>, _user:AuthUser)
-> Result<Json<Vec<dental_service::Model>>, StatusCode>{
    let rows = dental_service::Entity::find()
        .filter(dental_service::Column::Active.eq(true))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows))
}