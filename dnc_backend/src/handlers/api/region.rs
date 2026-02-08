use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::handlers::{ListQuery, PageResponse};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::handlers::structs::AuthUser;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

use crate::entities::region;

#[derive(Debug, Deserialize)]
pub struct RegionListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct RegionRow {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRegionRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchRegionRequest {
    pub name: Option<String>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_regions(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: auth stays, but no permission check per your request
    Query(params): Query<RegionListQuery>,
) -> Result<Json<PageResponse<RegionRow>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let page0: u64 = page.saturating_sub(1) as u64;

    let q = region::Entity::find()
        .select_only()
        .columns([region::Column::Id, region::Column::Name])
        .order_by_asc(region::Column::Name)
        .into_model::<RegionRow>();

    let paginator = q.paginate(&state.db, page_size as u64);

    let total_items = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count regions: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let total_pages = paginator.num_pages().await.map_err(|e| {
        tracing::error!("Failed to count region pages: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch regions: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PageResponse {
        page,
        page_size,
        total_items,
        total_pages,
        items,
    }))
}

#[instrument(skip(state), err(Debug))]
pub async fn get_region_by_id(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Path(id): Path<i32>,
) -> Result<Json<RegionRow>, StatusCode> {
    let row = region::Entity::find()
        .select_only()
        .columns([region::Column::Id, region::Column::Name])
        .filter(region::Column::Id.eq(id))
        .into_model::<RegionRow>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get region by id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

#[instrument(skip(state), err(Debug))]
pub async fn post_region(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateRegionRequest>,
) -> Result<Json<RegionRow>, StatusCode> {
    // KEEP: permission check for POST
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "region",
        PermissionActionEnum::Create,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut am: region::ActiveModel = Default::default();
    am.name = Set(payload.name);

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert region: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(RegionRow {
        id: created.id,
        name: created.name,
    }))
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_region(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchRegionRequest>,
) -> Result<Json<RegionRow>, StatusCode> {
    // KEEP: permission check for PATCH
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "region",
        PermissionActionEnum::Update,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    let model = region::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find region id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut am: region::ActiveModel = model.into();
    if let Some(name) = payload.name {
        am.name = Set(name);
    }

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update region id={id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(RegionRow {
        id: updated.id,
        name: updated.name,
    }))
}
