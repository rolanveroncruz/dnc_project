use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{FixedOffset, Utc};
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

use crate::entities::{city, dental_clinic, region, state};

#[derive(Debug, Deserialize)]
pub struct DentalClinicListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    // Optional filters
    pub city_id: Option<i32>,
    pub state_id: Option<i32>,  // via join city -> state
    pub region_id: Option<i32>, // via join city -> state -> region
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct DentalClinicRow {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub city_id: Option<i32>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub active: Option<bool>,
    pub last_modified_by: String,
    pub last_modified_on: sea_orm::entity::prelude::DateTimeWithTimeZone,

    // Expanded fields (nullable because city_id is nullable)
    pub city_name: Option<String>,
    pub state_id: Option<i32>,
    pub state_name: Option<String>,
    pub region_id: Option<i32>,
    pub region_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDentalClinicRequest {
    pub name: String,
    pub address: String,
    pub city_id: Option<i32>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDentalClinicRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city_id: Option<i32>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub active: Option<bool>,
}

fn now_utc_tz() -> sea_orm::entity::prelude::DateTimeWithTimeZone {
    // DateTimeWithTimeZone is chrono::DateTime<FixedOffset>
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

#[instrument(skip(state), err(Debug))]
pub async fn get_dental_clinics(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Query(params): Query<DentalClinicListQuery>,
) -> Result<Json<PageResponse<DentalClinicRow>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let page0: u64 = page.saturating_sub(1);

    // Base query + joins for expanded fields
    let mut q = dental_clinic::Entity::find()
        .join(JoinType::LeftJoin, dental_clinic::Relation::City.def())
        .join(JoinType::LeftJoin, city::Relation::State.def())
        .join(JoinType::LeftJoin, state::Relation::Region.def())
        .select_only()
        .columns([
            dental_clinic::Column::Id,
            dental_clinic::Column::Name,
            dental_clinic::Column::Address,
            dental_clinic::Column::CityId,
            dental_clinic::Column::Remarks,
            dental_clinic::Column::ContactNumbers,
            dental_clinic::Column::Active,
            dental_clinic::Column::LastModifiedBy,
            dental_clinic::Column::LastModifiedOn,
        ])
        // Expanded columns
        .column_as(city::Column::Name, "city_name")
        .column_as(state::Column::Id, "state_id")
        .column_as(state::Column::Name, "state_name")
        .column_as(region::Column::Id, "region_id")
        .column_as(region::Column::Name, "region_name")
        .order_by_asc(dental_clinic::Column::Name);

    // Filters
    if let Some(city_id) = params.city_id {
        q = q.filter(dental_clinic::Column::CityId.eq(city_id));
    }
    if let Some(active) = params.active {
        q = q.filter(dental_clinic::Column::Active.eq(active));
    }
    if let Some(state_id) = params.state_id {
        q = q.filter(state::Column::Id.eq(state_id));
    }
    if let Some(region_id) = params.region_id {
        q = q.filter(region::Column::Id.eq(region_id));
    }

    let q = q.into_model::<DentalClinicRow>();
    let paginator = q.paginate(&state.db, page_size);

    let total_items = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count dental clinics: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let total_pages = paginator.num_pages().await.map_err(|e| {
        tracing::error!("Failed to count dental clinic pages: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch dental clinics: {e:?}");
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
pub async fn get_dental_clinic_by_id(
    State(state): State<AppState>,
    _user: AuthUser, // NOTE: no permission check for GET
    Path(id): Path<i32>,
) -> Result<Json<DentalClinicRow>, StatusCode> {
    let row = dental_clinic::Entity::find()
        .join(JoinType::LeftJoin, dental_clinic::Relation::City.def())
        .join(JoinType::LeftJoin, city::Relation::State.def())
        .join(JoinType::LeftJoin, state::Relation::Region.def())
        .select_only()
        .columns([
            dental_clinic::Column::Id,
            dental_clinic::Column::Name,
            dental_clinic::Column::Address,
            dental_clinic::Column::CityId,
            dental_clinic::Column::Remarks,
            dental_clinic::Column::ContactNumbers,
            dental_clinic::Column::Active,
            dental_clinic::Column::LastModifiedBy,
            dental_clinic::Column::LastModifiedOn,
        ])
        .column_as(city::Column::Name, "city_name")
        .column_as(state::Column::Id, "state_id")
        .column_as(state::Column::Name, "state_name")
        .column_as(region::Column::Id, "region_id")
        .column_as(region::Column::Name, "region_name")
        .filter(dental_clinic::Column::Id.eq(id))
        .into_model::<DentalClinicRow>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get dental clinic by id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}

#[instrument(skip(state), err(Debug))]
pub async fn post_dental_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateDentalClinicRequest>,
) -> Result<Json<DentalClinicRow>, StatusCode> {
    // KEEP: permission check for POST
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dental_clinic",
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

    // Validate city_id if provided
    if let Some(city_id) = payload.city_id {
        let exists = city::Entity::find_by_id(city_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to validate city_id={city_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .is_some();

        if !exists {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let now = now_utc_tz();

    let mut am: dental_clinic::ActiveModel = Default::default();
    am.name = Set(payload.name);
    am.address = Set(payload.address);
    am.city_id = Set(payload.city_id);
    am.remarks = Set(payload.remarks);
    am.contact_numbers = Set(payload.contact_numbers);
    am.active = Set(payload.active);

    // CHANGE THIS: pick the correct field from your AuthUser claims
    // Examples: user.claims.email.clone(), user.claims.name.clone(), user.claims.username.clone()
    am.last_modified_by = Set(user.claims.email.clone());
    am.last_modified_on = Set(now);

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert dental clinic: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Return expanded row
    get_dental_clinic_by_id(State(state), user, Path(created.id)).await
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_dental_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchDentalClinicRequest>,
) -> Result<Json<DentalClinicRow>, StatusCode> {
    // KEEP: permission check for PATCH
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dental_clinic",
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

    // Validate city_id if changing it
    if let Some(city_id) = payload.city_id {
        let exists = city::Entity::find_by_id(city_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to validate new city_id={city_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .is_some();

        if !exists {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let model = dental_clinic::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find dental clinic id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let now = now_utc_tz();

    let mut am: dental_clinic::ActiveModel = model.into();

    if let Some(name) = payload.name {
        am.name = Set(name);
    }
    if let Some(address) = payload.address {
        am.address = Set(address);
    }
    if let Some(city_id) = payload.city_id {
        am.city_id = Set(Some(city_id)); // NOTE: this PATCH cannot clear city_id to NULL
    }
    if let Some(remarks) = payload.remarks {
        am.remarks = Set(Some(remarks));
    }
    if let Some(contact_numbers) = payload.contact_numbers {
        am.contact_numbers = Set(Some(contact_numbers));
    }
    if let Some(active) = payload.active {
        am.active = Set(Some(active));
    }

    // CHANGE THIS: pick the correct field from your AuthUser claims
    am.last_modified_by = Set(user.claims.email.clone());
    am.last_modified_on = Set(now);

    am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update dental clinic id={id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_dental_clinic_by_id(State(state), user, Path(id)).await
}
