use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::handlers::{ListQuery, PageResponse};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::handlers::structs::AuthUser;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

use crate::entities::{region, state};

#[derive(Debug, Deserialize)]
pub struct StateListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    // CHANGE: optional filter by region
    pub region_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct StateRow {
    pub id: i32,
    pub name: String,
    pub region_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateStateRequest {
    pub name: String,
    pub region_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PatchStateRequest {
    pub name: Option<String>,
    pub region_id: Option<i32>,
}

#[instrument(skip(state), err(Debug))]
pub async fn get_states(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Query(params): Query<StateListQuery>,
) -> Result<Json<PageResponse<StateRow>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let page0: u64 = page.saturating_sub(1) as u64;

    let mut q = state::Entity::find()
        .select_only()
        .columns([state::Column::Id, state::Column::Name, state::Column::RegionId])
        .order_by_asc(state::Column::Name);

    if let Some(region_id) = params.region_id {
        q = q.filter(state::Column::RegionId.eq(region_id));
    }

    let q = q.into_model::<StateRow>();

    let paginator = q.paginate(&state.db, page_size as u64);

    let total_items = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count states: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let total_pages = paginator.num_pages().await.map_err(|e| {
        tracing::error!("Failed to count state pages: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch states: {e:?}");
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
pub async fn get_state_by_id(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Path(id): Path<i32>,
) -> Result<Json<StateRow>, StatusCode> {
    let row = state::Entity::find()
        .select_only()
        .columns([state::Column::Id, state::Column::Name, state::Column::RegionId])
        .filter(state::Column::Id.eq(id))
        .into_model::<StateRow>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get state by id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

#[instrument(skip(state), err(Debug))]
pub async fn post_state(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateStateRequest>,
) -> Result<Json<StateRow>, StatusCode> {
    // KEEP: permission check for POST
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "state",
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

    // NEW: parent validation (region must exist)
    let region_exists = region::Entity::find_by_id(payload.region_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to validate region_id={}: {e:?}", payload.region_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();
    if !region_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut am: state::ActiveModel = Default::default();
    am.name = Set(payload.name);
    am.region_id = Set(payload.region_id);

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert state: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(StateRow {
        id: created.id,
        name: created.name,
        region_id: created.region_id,
    }))
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_state(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchStateRequest>,
) -> Result<Json<StateRow>, StatusCode> {
    // KEEP: permission check for PATCH
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "state",
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

    let model = state::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find state id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // If changing region_id, validate new region exists
    if let Some(new_region_id) = payload.region_id {
        let region_exists = region::Entity::find_by_id(new_region_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to validate new region_id={new_region_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .is_some();
        if !region_exists {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let mut am: state::ActiveModel = model.into();

    if let Some(name) = payload.name {
        am.name = Set(name);
    }
    if let Some(region_id) = payload.region_id {
        am.region_id = Set(region_id);
    }

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update state id={id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(StateRow {
        id: updated.id,
        name: updated.name,
        region_id: updated.region_id,
    }))
}
