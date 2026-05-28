use axum::{extract::{Query, State}, http::StatusCode, Json};
use chrono::Utc;
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, FromQueryResult,
              JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
              Set,
};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{role_permission, role, permission, data_object};
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

//region: get_role_permissions
#[derive(Debug, Deserialize)]
pub struct RolePermissionListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct RolePermissionRow {
    pub id: i32,
    pub role_id: i32,
    pub role: String,
    pub object_id: i32,
    pub object: String,
    pub action: String,
    pub active: bool,
    pub last_modified_by: Option<String>,
    pub last_modified_on: chrono::DateTime<chrono::Utc>, // adjust type to your column type
}

#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_role_permissions(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<RolePermissionListQuery>,
) -> Result<Json<PageResponse<RolePermissionRow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "role_permission",
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
    let page_size = params.base.page_size.unwrap_or(200).clamp(1, 200);
    let active = params.base.active.unwrap_or(true);
    let page0:u64 = page.saturating_sub(1);

    let sort = params.base.sort.as_deref().unwrap_or("name");
    let order = params.base.order.as_deref().unwrap_or("asc");


    // 3. Build the query (JOIN to dental_service_type)
    let mut query = role_permission::Entity::find()
        .join(JoinType::InnerJoin, role_permission::Relation::Role.def())
        .join(JoinType::InnerJoin, role_permission::Relation::Permission.def())
        .join(JoinType::LeftJoin, permission::Relation::DataObject.def())
        .filter(role_permission::Column::Active.eq(active));


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
        .select_only()
        .column(role_permission::Column::Id)
        .column(role_permission::Column::RoleId)
        .column_as(role::Column::Name, "role")
        .column_as(permission::Column::DataObjectId, "object_id")
        .column_as(data_object::Column::Name, "object")
        .column_as(permission::Column::Action, "action")
        .column(role_permission::Column::Active)
        .column_as(role_permission::Column::LastModifiedBy, "last_modified_by")
        .column_as(role_permission::Column::LastModifiedOn, "last_modified_on")
        .into_model::<RolePermissionRow>();

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
    tracing::info!("user {} fetched page {} of {} role permissions", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}


//endregion: get_role_permissions

// region: create_role_permission
#[derive(Debug, Deserialize)]
pub struct CreateRolePermissionRequest {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateRolePermissionResponse {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub active: bool,
}

#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn create_role_permission(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateRolePermissionRequest>,
) -> Result<(StatusCode, Json<CreateRolePermissionResponse>), StatusCode> {
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "role_permission",
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

    // 2. Validate role exists
    let role_exists = role::Entity::find_by_id(payload.role_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check role existence: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !role_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 3. Validate permission exists
    let permission_exists = permission::Entity::find_by_id(payload.permission_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission existence: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !permission_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 4. Prevent duplicate role-permission pair
    let existing = role_permission::Entity::find()
        .filter(role_permission::Column::RoleId.eq(payload.role_id))
        .filter(role_permission::Column::PermissionId.eq(payload.permission_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check duplicate role_permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 5. Insert
    let now = Utc::now().fixed_offset();

    let new_role_permission = role_permission::ActiveModel {
        role_id: Set(payload.role_id),
        permission_id: Set(payload.permission_id),
        active: Set(true),
        last_modified_by: Set(user.claims.email.clone()),
        last_modified_on: Set(now),
        ..Default::default()
    };

    let inserted = new_role_permission
        .insert(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create role_permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!(
        "user {} created role_permission id {}",
        user.claims.email,
        inserted.id
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateRolePermissionResponse {
            id: inserted.id,
            role_id: inserted.role_id,
            permission_id: inserted.permission_id,
            active: inserted.active,
        }),
    ))
}


// endregion: create_role_permission
