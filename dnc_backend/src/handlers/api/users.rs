use argon2::Argon2;
use axum::{extract::{Query, Path, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ActiveModelTrait, ColumnTrait,  Condition, EntityTrait,IntoActiveModel, FromQueryResult, JoinType, Order,
              PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{user, role};
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;
use chrono::Utc;
use password_hash::rand_core::OsRng;
use password_hash::SaltString;

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role_id: i32,
    pub role: String,
    pub active: bool,
    pub last_modified_by: Option<String>,
    pub last_modified_on: chrono::DateTime<chrono::Utc>, // adjust type to your column type
}

#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_users(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<UserListQuery>,
) -> Result<Json<PageResponse<UserRow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "user",
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
    let mut query = user::Entity::find()
        .join(JoinType::LeftJoin, user::Relation::Role.def())
        .filter(user::Column::Active.eq(active))
        .filter(user::Column::Id.ne(2));


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
            Condition::any().add(Expr::col(user::Column::Name).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(user::Column::Id, sort_order),
        "name" => query.order_by(user::Column::Name, sort_order),
        "lastModifiedBy" | "last_modified_by" => query.order_by(user::Column::LastModifiedBy, sort_order),
        "lastModifiedOn" | "last_modified_on" => query.order_by(user::Column::LastModifiedOn, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // 5. Select only the columns we want + alias type name as 'type_name'
    let query = query
        .select_only()
        .column(user::Column::Id)
        .column(user::Column::Name)
        .column(user::Column::Active)
        .column(user::Column::Email)
        .column_as(role::Column::Id, "role_id")
        .column_as(role::Column::Name, "role")
        .column_as(user::Column::LastModifiedBy, "last_modified_by")
        .column_as(user::Column::LastModifiedOn, "last_modified_on")
        .into_model::<UserRow>();

    // 6. Paginate
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
    tracing::info!("user {} fetched page {} of {} users", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: i32,
    #[serde(default = "default_active_true")]
    pub active: bool,
}

fn default_active_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct PatchUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role_id: i32,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: sea_orm::prelude::DateTimeWithTimeZone,
}

impl From<user::Model> for UserResponse {
    fn from(m: user::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            email: m.email,
            role_id: m.role_id,
            active: m.active,
            last_modified_by: m.last_modified_by,
            last_modified_on: m.last_modified_on,
        }
    }
}

use argon2::PasswordHasher;/// Replace this with whatever you already use (argon2/bcrypt/etc).
fn hash_password(plain: &str) -> Result<String, ()> {
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let argon2 = Argon2::default();
    let password_hash= argon2
        .hash_password(plain.as_bytes(), &salt)
        .unwrap()
        .to_string();
    Ok(format!("{password_hash}"))
}

#[instrument(skip(state, body), err(Debug))]
pub async fn post_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        auth.claims.role_id,
        "user",
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
    if body.name.trim().is_empty() || body.email.trim().is_empty() || body.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 3) Ensure email unique
    let existing = user::Entity::find()
        .filter(user::Column::Email.eq(body.email.trim()))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to query existing email: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 4) Hash password
    let password_hash = hash_password(&body.password).map_err(|x|{
        tracing::error!("Failed to hash password: {x:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    } )?;

    // 5) Insert
    let now = Utc::now().fixed_offset();
    let last_modified_by = auth.claims.email.clone(); // <- adjust to your claims fields

    let am = user::ActiveModel {
        name: Set(body.name.trim().to_string()),
        email: Set(body.email.trim().to_string()),
        password: Set(password_hash),
        role_id: Set(body.role_id),
        active: Set(body.active),
        last_modified_by: Set(last_modified_by),
        last_modified_on: Set(now),
        ..Default::default()
    };

    let inserted = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert user: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(inserted.into()))
}

#[instrument(skip(state, body), err(Debug))]
pub async fn patch_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<i32>,
    Json(body): Json<PatchUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        auth.claims.role_id,
        "user",
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
    let model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch user {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 3) If email changes, enforce unique
    if let Some(new_email) = body.email.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        if new_email != model.email {
            let email_taken = user::Entity::find()
                .filter(user::Column::Email.eq(new_email))
                .one(&state.db)
                .await
                .map_err(|e| {
                    tracing::error!("Failed checking email uniqueness: {e:?}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            if email_taken.is_some() {
                return Err(StatusCode::CONFLICT);
            }
        }
    }

    // 4) Prepare update
    let mut am: user::ActiveModel = Default::default();
    am.id = Set(model.id);

    if let Some(name) = body.name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        am.name = Set(name.to_string());
    }

    if let Some(email) = body.email.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        am.email = Set(email.to_string());
    }

    if let Some(role_id) = body.role_id {
        am.role_id = Set(role_id);
    }

    if let Some(active) = body.active {
        am.active = Set(active);
    }

    if let Some(pw) = body
        .password
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let password_hash = hash_password(pw).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        am.password = Set(password_hash);
    }

    let now = Utc::now().fixed_offset();
    am.last_modified_by = Set(auth.claims.email.clone()); // <- adjust to your claims fields
    am.last_modified_on = Set(now);

    // 5) Update
    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update user {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated.into()))
}