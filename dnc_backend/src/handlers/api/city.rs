#![allow(dead_code, unused_imports)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::handlers::{ListQuery, PageResponse};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::handlers::structs::AuthUser;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

use crate::entities::{city, state};

#[derive(Debug, Deserialize)]
pub struct CityListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    // CHANGE: filter by parent(s)
    pub state_id: Option<i32>,
    pub region_id: Option<i32>, // implemented via join city -> state
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct CityRow {
    pub id: i32,
    pub name: String,
    pub state_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCityRequest {
    pub name: String,
    pub state_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PatchCityRequest {
    pub name: Option<String>,
    pub state_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_cities(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Query(params): Query<CityListQuery>,
) -> Result<Json<PageResponse<CityRow>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let page0: u64 = page.saturating_sub(1) as u64;

    // Base select
    let mut q = city::Entity::find()
        .select_only()
        .columns([city::Column::Id, city::Column::Name, city::Column::StateId])
        .order_by_asc(city::Column::Name);

    if let Some(state_id) = params.state_id {
        q = q.filter(city::Column::StateId.eq(state_id));
    }

    // OPTIONAL: filter by region_id via join (city -> state)
    if let Some(region_id) = params.region_id {
        q = q
            .join(JoinType::InnerJoin, city::Relation::State.def())
            .filter(state::Column::RegionId.eq(region_id));
    }

    let q = q.into_model::<CityRow>();
    let paginator = q.paginate(&state.db, page_size as u64);

    let total_items = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count cities: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let total_pages = paginator.num_pages().await.map_err(|e| {
        tracing::error!("Failed to count city pages: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch cities: {e:?}");
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
pub async fn get_city_by_id(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Path(id): Path<i32>,
) -> Result<Json<CityRow>, StatusCode> {
    let row = city::Entity::find()
        .select_only()
        .columns([city::Column::Id, city::Column::Name, city::Column::StateId])
        .filter(city::Column::Id.eq(id))
        .into_model::<CityRow>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get city by id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

#[instrument(skip(state), err(Debug))]
pub async fn post_city(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateCityRequest>,
) -> Result<Json<CityRow>, StatusCode> {
    // KEEP: permission check for POST
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "city",
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

    // NEW: parent validation (state must exist)
    let state_exists = state::Entity::find_by_id(payload.state_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to validate state_id={}: {e:?}", payload.state_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();
    if !state_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut am: city::ActiveModel = Default::default();
    am.name = Set(payload.name);
    am.state_id = Set(payload.state_id);

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert city: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CityRow {
        id: created.id,
        name: created.name,
        state_id: created.state_id,
    }))
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_city(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchCityRequest>,
) -> Result<Json<CityRow>, StatusCode> {
    // KEEP: permission check for PATCH
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "city",
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

    let model = city::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find city id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // If changing state_id, validate new state exists
    if let Some(new_state_id) = payload.state_id {
        let state_exists = state::Entity::find_by_id(new_state_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to validate new state_id={new_state_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .is_some();
        if !state_exists {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let mut am: city::ActiveModel = model.into();

    if let Some(name) = payload.name {
        am.name = Set(name);
    }
    if let Some(state_id) = payload.state_id {
        am.state_id = Set(state_id);
    }

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update city id={id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CityRow {
        id: updated.id,
        name: updated.name,
        state_id: updated.state_id,
    }))
}
