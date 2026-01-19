use sea_orm::{ColumnTrait, NotSet};
use axum::{extract::{Query, Path, State}, http::StatusCode, Json};
use chrono::{FixedOffset, Utc};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ ActiveModelTrait, Condition, EntityTrait, FromQueryResult,  Order,
              PaginatorTrait, QueryFilter, QueryOrder, Set };
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{clinic_capability };
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct ClinicCapabilityListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct ClinicCapabilityRow {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: chrono::DateTime<chrono::Utc>
}

#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_clinic_capabilities(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<ClinicCapabilityListQuery>,
) -> Result<Json<PageResponse<ClinicCapabilityRow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capability",
        PermissionActionEnum::Read,).
        await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !has_permission{
        return Err(StatusCode::FORBIDDEN);
    };

    // 2. Defaults + basic validation/clamping
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let active = params.base.active.unwrap_or(true);
    let page0:u64 = page.saturating_sub(1);

    let sort = params.base.sort.as_deref().unwrap_or("name");
    let order = params.base.order.as_deref().unwrap_or("asc");


    // 3. Build the query (JOIN to dental_service_type)
    let mut query = clinic_capability::Entity::find()
        .filter(clinic_capability::Column::Active.eq(active));

    // 4. Safe sort mapping (never trust raw column names from the client!)
    let sort_order = match order {
        "asc" => Order::Asc,
        "desc" => Order::Desc,
        _ => return Err(StatusCode::BAD_REQUEST),
    };


    // q=clean (search by name; using ILIKE for Postgres-ish case-insensitive search)
    if let Some(q) = params.base.q.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        let pattern = format!("%{}%", q);
        query = query.filter(
            Condition::any().add(Expr::col(clinic_capability::Column::Name).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(clinic_capability::Column::Id, sort_order),
        "name" => query.order_by(clinic_capability::Column::Name, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let query = query
        .into_model::<ClinicCapabilityRow>();

    let paginator = query.paginate(&state.db, page_size);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|_| {
            tracing::error!("Failed to count items in paginator");
            StatusCode::INTERNAL_SERVER_ERROR
            })?;


    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|_| {
            tracing::error!("Failed to count pages in paginator");
            StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let items = paginator
        .fetch_page(page0)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch page {page0} from paginator: {e:?}" );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    tracing::info!("user {} fetched page {} of {} clinic capabilities", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}


fn now_utc_fixed() -> sea_orm::prelude::DateTimeWithTimeZone {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

fn normalize_name(name: &str) -> Option<String> {
    let s = name.trim();
    if s.is_empty() { None } else { Some(s.to_string()) }
}

#[derive(Debug, Deserialize)]
pub struct CreateClinicCapabilityPayload {
    pub name: String,
    pub active: Option<bool>, // default true
}

#[derive(Debug, Deserialize)]
pub struct PatchClinicCapabilityPayload {
    pub name: Option<String>,
    pub active: Option<bool>,
}

/// POST /clinic_capabilities
#[instrument(skip(state), err(Debug))]
pub async fn post_clinic_capability(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateClinicCapabilityPayload>,
) -> Result<(StatusCode, Json<clinic_capability::Model>), StatusCode> {
    // Optional RBAC
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capability",
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

    let name = normalize_name(&payload.name).ok_or(StatusCode::BAD_REQUEST)?;
    let active = payload.active.unwrap_or(true);

    let now = now_utc_fixed();
    let last_modified_by = user.claims.email.clone(); // adjust if your claims differ

    let am = clinic_capability::ActiveModel {
        id: NotSet, // <-- identity generated by DB
        name: Set(name),
        active: Set(active),
        last_modified_by: Set(last_modified_by),
        last_modified_on: Set(now),
    };
    tracing::info!("Creating clinic capability: {:?}", am);
    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("DB error (insert clinic_capability): {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(created)))
}

/// PATCH /clinic_capabilities/{id}
#[instrument(skip(state), err(Debug))]
pub async fn patch_clinic_capability(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchClinicCapabilityPayload>,
) -> Result<Json<clinic_capability::Model>, StatusCode> {
    // Optional RBAC
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capability",
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

    if id <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let model = clinic_capability::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("DB error (find_by_id): {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut am: clinic_capability::ActiveModel = model.into();

    if let Some(name) = payload.name.as_deref() {
        let name = normalize_name(name).ok_or(StatusCode::BAD_REQUEST)?;
        am.name = Set(name);
    }
    if let Some(active) = payload.active {
        am.active = Set(active);
    }

    // bump audit fields
    am.last_modified_by = Set(user.claims.email.clone());
    am.last_modified_on = Set(now_utc_fixed());

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("DB error (update clinic_capability): {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated))
}