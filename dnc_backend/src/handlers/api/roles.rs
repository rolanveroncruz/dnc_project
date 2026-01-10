use axum::{extract::{Query, Path, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ActiveModelTrait, ColumnTrait,  Condition, EntityTrait, FromQueryResult, Order,
              PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{role };
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct RoleListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct RoleRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub last_modified_by: Option<String>,
    pub last_modified_on: chrono::DateTime<chrono::Utc>, // adjust type to your column type
}

#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_roles(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<RoleListQuery>,
) -> Result<Json<PageResponse<RoleRow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "role",
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
    let mut query = role::Entity::find()
        .filter(role::Column::Active.eq(active));


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
            Condition::any().add(Expr::col(role::Column::Name).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(role::Column::Id, sort_order),
        "name" => query.order_by(role::Column::Name, sort_order),
        "lastModifiedBy" | "last_modified_by" => query.order_by(role::Column::LastModifiedBy, sort_order),
        "lastModifiedOn" | "last_modified_on" => query.order_by(role::Column::LastModifiedOn, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // 5. Select only the columns we want + alias type name as 'type_name'
    let query = query
        .into_model::<RoleRow>();

    // 6. Paginate
    let paginator = query.paginate(&state.db, page_size);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| {
            tracing::error!("Failed to count items in paginator:{e:?}");
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
    tracing::info!("user {} fetched page {} of {} roles", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}


#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreateRoleResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: DateTimeWithTimeZone,
}

#[instrument(skip(state), err(Debug))]
pub async fn create_role(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<CreateRoleResponse>), StatusCode> {

    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "role",
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

    // 2) Basic validation
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let description = payload.description.trim();
    if description.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Choose what you want to store here (email, name, etc.)
    let last_modified_by = user.claims.email.clone(); // <- adjust if your claims differ
    let now = chrono::Utc::now().into();

    // 3) Insert
    let active_model = role::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.to_string()),
        active: Set(payload.active.unwrap_or(true)),
        last_modified_by: Set(last_modified_by.clone()),
        last_modified_on: Set(now),
        ..Default::default()
    };

    // For Postgres, this returns the inserted row via RETURNING.
    let inserted: role::Model = role::Entity::insert(active_model)
        .exec_with_returning(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert role: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRoleResponse {
            id: inserted.id,
            name: inserted.name,
            description: inserted.description,
            active: inserted.active,
            last_modified_by: inserted.last_modified_by,
            last_modified_on: inserted.last_modified_on,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct PatchRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PatchRoleResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: DateTimeWithTimeZone,
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_role(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchRoleRequest>,
) -> Result<Json<PatchRoleResponse>, StatusCode> {

    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "role",
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

    // 2) Load existing
    let existing = role::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to query role id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 3) Validate provided fields (only if present)
    if let Some(name) = payload.name.as_deref() {
        if name.trim().is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    if let Some(desc) = payload.description.as_deref() {
        if desc.trim().is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // 4) Apply patch via ActiveModel
    let mut am: role::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        am.name = Set(name.trim().to_string());
    }
    if let Some(description) = payload.description {
        am.description = Set(description.trim().to_string());
    }
    if let Some(active) = payload.active {
        am.active = Set(active);
    }

    // audit fields
    let last_modified_by = user.claims.email.clone(); // adjust if your claims differ
    let now: DateTimeWithTimeZone = chrono::Utc::now().into();
    am.last_modified_by = Set(last_modified_by);
    am.last_modified_on = Set(now);

    // 5) Persist + return updated model
    let updated: role::Model = am
        .update(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update role id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(PatchRoleResponse {
        id: updated.id,
        name: updated.name,
        description: updated.description,
        active: updated.active,
        last_modified_by: updated.last_modified_by,
        last_modified_on: updated.last_modified_on,
    }))
}
