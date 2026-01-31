use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::Deserialize;
use tracing::instrument;

use crate::AppState;

use crate::entities::{city, province};

#[derive(Debug, Deserialize)]
pub struct ProvinceListQuery {
    /// Optional: /provinces?region_id=1
    pub region_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_provinces(
    State(state): State<AppState>,
    Query(params): Query<ProvinceListQuery>,
) -> Result<Json<Vec<province::Model>>, StatusCode> {
    let mut q = province::Entity::find()
        .order_by_asc(province::Column::Name);

    if let Some(region_id) = params.region_id {
        q = q.filter(province::Column::RegionId.eq(region_id));
    }

    let rows = q
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch provinces: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}

#[instrument(skip(state), err(Debug))]
pub async fn get_cities_by_province(
    State(state): State<AppState>,
    Path(province_id): Path<i32>,
) -> Result<Json<Vec<city::Model>>, StatusCode> {
    // Ensure province exists (so we can return 404 instead of empty list)
    let exists = province::Entity::find_by_id(province_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch province {province_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // DB-side filter + sort
    let cities = city::Entity::find()
        .filter(city::Column::ProvinceId.eq(province_id))
        .order_by_asc(city::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch cities for province {province_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(cities))
}
