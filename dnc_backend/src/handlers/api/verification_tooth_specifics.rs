use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder};
use serde::Serialize;

use crate::AppState;
use crate::entities::{tooth_service_type, tooth_surface};

#[derive(Debug, Serialize)]
pub struct ToothServiceTypeResponse {
    pub id: i32,
    pub name: String,
}

pub async fn get_tooth_service_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<ToothServiceTypeResponse>>, (StatusCode, String)> {
    let rows = tooth_service_type::Entity::find()
        .all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch tooth service types: {}", e),
            )
        })?;

    let response = rows
        .into_iter()
        .map(|row| ToothServiceTypeResponse {
            id: row.id,
            name: row.name,
        })
        .collect();

    Ok(Json(response))
}


#[derive(Debug, Serialize)]
pub struct ToothSurfaceResponse {
    pub id: i32,
    pub name: String,
}
pub async fn get_tooth_surfaces(
    State(state): State<AppState>,
) -> Result<Json<Vec<ToothSurfaceResponse>>, (StatusCode, String)> {
    let rows = tooth_surface::Entity::find()
        .order_by_asc(tooth_surface::Column::Id)
        .all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch tooth surfaces: {}", e),
            )
        })?;

    let response = rows
        .into_iter()
        .map(|row| ToothSurfaceResponse {
            id: row.id,
            name: row.name,
        })
        .collect();

    Ok(Json(response))
}