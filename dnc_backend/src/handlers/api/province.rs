use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::{city, province};

//
// ---- List response (paging)
//

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,      // 1-based
    pub page_size: Option<u64>, // clamp server-side
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub page: u64, // 1-based
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct ProvinceListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    /// Optional: /provinces?region_id=1
    pub region_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_provinces(
    State(state): State<AppState>,
    Query(params): Query<ProvinceListQuery>,
) -> Result<Json<PageResponse<province::Model>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(650).clamp(1, 1000);
    let page0 = page.saturating_sub(1);

    let mut q = province::Entity::find().order_by_asc(province::Column::Name);

    if let Some(region_id) = params.region_id {
        q = q.filter(province::Column::RegionId.eq(region_id));
    }

    let paginator = q.paginate(&state.db, page_size);

    let total = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count provinces: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch provinces page={page} size={page_size}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PageResponse {
        page,
        page_size,
        total,
        items,
    }))
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
