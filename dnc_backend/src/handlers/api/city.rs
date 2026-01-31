use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, JoinType,
};
use serde::Deserialize;
use tracing::instrument;

use crate::AppState;
use crate::entities::{city, province};

#[derive(Debug, Deserialize)]
pub struct CityListQuery {
    /// Optional: /cities?province_id=10
    pub province_id: Option<i32>,

    /// Optional: /cities?region_id=1 (via join city -> province)
    pub region_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_cities(
    State(state): State<AppState>,
    Query(params): Query<CityListQuery>,
) -> Result<Json<Vec<city::Model>>, StatusCode> {
    // Base: all cities, sorted by name
    let mut q = city::Entity::find().order_by_asc(city::Column::Name);

    // Filter: by province_id (simple, no join)
    if let Some(province_id) = params.province_id {
        q = q.filter(city::Column::ProvinceId.eq(province_id));
    }

    // Filter: by region_id (requires join city -> province)
    if let Some(region_id) = params.region_id {
        q = q
            .join(JoinType::InnerJoin, city::Relation::Province.def())
            .filter(province::Column::RegionId.eq(region_id));
    }

    let rows = q
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch cities: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
