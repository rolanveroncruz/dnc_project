use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, JoinType, PaginatorTrait, RelationTrait, QuerySelect};
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
pub struct CityListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    /// Optional: /cities?province_id=10
    pub province_id: Option<i32>,

    /// Optional: /cities?region_id=1 (via join city -> province)
    pub region_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_cities(
    State(state): State<AppState>,
    Query(params): Query<CityListQuery>,
) -> Result<Json<PageResponse<city::Model>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(650).clamp(1, 1000);
    let page0 = page.saturating_sub(1);

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

    let paginator = q.paginate(&state.db, page_size);

    let total = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count cities: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch cities page={page} size={page_size}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PageResponse {
        page,
        page_size,
        total,
        items,
    }))
}
