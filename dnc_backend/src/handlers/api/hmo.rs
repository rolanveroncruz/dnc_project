use axum::{extract::{Query, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet,ColumnTrait, Condition, EntityTrait, FromQueryResult, Order,
              PaginatorTrait, QueryFilter, QueryOrder, Set};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{hmo};
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct HMOListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct HMORow {
    pub id: i32,
    pub short_name: String,
    pub long_name: String,
    pub address: Option<String>,
    pub tax_account_number: Option<String>,
    pub contact_nos: Option<String>,
    pub expect_a_master_list: Option<bool>,
    pub active: bool,
    pub last_endorsement_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_collection_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_modified_by: String,
    pub last_modified_on: chrono::DateTime<chrono::Utc>,
}




#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_hmos(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<HMOListQuery>,
) -> Result<Json<PageResponse<HMORow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "hmo",
        PermissionActionEnum::Read,).
        await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !has_permission{
        return Err(StatusCode::FORBIDDEN);
    };

    // 2. Defaults + basic validation/clamping
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let active = params.base.active.unwrap_or(true);

    let sort = params.base.sort.as_deref().unwrap_or("name");
    println!("sort: {}", sort);
    let order = params.base.order.as_deref().unwrap_or("asc");
    println!("order: {}", order);

    // 3. Build the query (JOIN to dental_service_type)
    let mut query = hmo::Entity::find()
        .filter(hmo::Column::Active.eq(active));

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
            Condition::any().add(Expr::col(hmo::Column::LongName).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(hmo::Column::Id, sort_order),
        "name" => query.order_by(hmo::Column::ShortName, sort_order),
        "longName" | "long_name" => query.order_by(hmo::Column::LongName, sort_order),
        "lastModifiedOn" | "last_modified_on" => query.order_by(hmo::Column::LastModifiedOn, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // 5. Select only the columns we want + alias type name as 'type_name'
    let query = query
        .into_model::<HMORow>();

    // 5. Paginate
    let paginator = query.paginate(&state.db, page_size);
    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| {
            eprintln!("Killed by: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|e| {
            eprintln!("Killed by: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let items = paginator
        .fetch_page(page-1)
        .await
        .map_err(|e| {
            eprintln!("Killed by: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!("user {} fetched page {} of {} dental services", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}
use axum::extract::Path;
use sea_orm::prelude::DateTimeWithTimeZone;

pub async fn get_hmo_by_id(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<HMORow>, StatusCode> {
    // Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "hmo",
        PermissionActionEnum::Read,
    )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    let row = hmo::Entity::find()
        .filter(hmo::Column::Id.eq(id))
        .into_model::<HMORow>()
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(row))
}


#[derive(Debug, Deserialize)]
pub struct CreateHmoRequest {
    pub short_name: String,
    pub long_name: String,
    pub address: Option<String>,
    pub tax_account_number: Option<String>,
    pub contact_nos: Option<String>,
    pub expect_a_master_list: Option<bool>,
    pub active: Option<bool>, // default to true if omitted
}

#[derive(Debug, Deserialize)]
pub struct PatchHmoRequest {
    pub short_name: Option<String>,
    pub long_name: Option<String>,

    // Option<Option<String>> lets PATCH explicitly clear a nullable column:
    // - omitted => don't change
    // - "address": null => set to NULL
    // - "address": "x" => set to "x"
    pub address: Option<Option<String>>,
    pub tax_account_number: Option<Option<String>>,
    pub contact_nos: Option<Option<String>>,

    pub expect_a_master_list: Option<Option<bool>>,

    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HmoResponse {
    pub id: i32,
    pub short_name: String,
    pub long_name: String,
    pub address: Option<String>,
    pub tax_account_number: Option<String>,
    pub contact_nos: Option<String>,
    pub expect_a_master_list: Option<bool>,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: DateTimeWithTimeZone,
}

impl From<hmo::Model> for HmoResponse {
    fn from(m: hmo::Model) -> Self {
        Self {
            id: m.id,
            short_name: m.short_name,
            long_name: m.long_name,
            address: m.address,
            tax_account_number: m.tax_account_number,
            contact_nos: m.contact_nos,
            expect_a_master_list: m.expect_a_master_list,
            active: m.active,
            last_modified_by: m.last_modified_by,
            last_modified_on: m.last_modified_on,
        }
    }
}

fn now_utc() -> DateTimeWithTimeZone {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
}

// ---------- POST /hmos ----------

#[instrument(skip(state, auth, body), err(Debug))]
pub async fn post_hmo(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateHmoRequest>,
) -> Result<(StatusCode, Json<HmoResponse>), StatusCode> {
    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        auth.claims.role_id,
        "hmo",
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
    let short_name = body.short_name.trim();
    let long_name = body.long_name.trim();
    if short_name.is_empty() || long_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 3) Create ActiveModel
    // NOTE: choose what you want to store as last_modified_by
    let last_modified_by = auth.claims.email.clone(); // <-- change if needed
    let now = now_utc();

    let active = body.active.unwrap_or(true);

    let am = hmo::ActiveModel {
        id: NotSet,
        short_name: Set(short_name.to_string()),
        long_name: Set(long_name.to_string()),
        address: Set(body.address),
        tax_account_number: Set(body.tax_account_number),
        contact_nos: Set(body.contact_nos),
        expect_a_master_list: Set(body.expect_a_master_list),
        active: Set(active),
        last_modified_by: Set(last_modified_by),
        last_modified_on: Set(now),
    };

    // 4) Insert
    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert hmo: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(created.into())))
}

// ---------- PATCH /hmos/:id ----------

#[instrument(skip(state, auth, body), err(Debug))]
pub async fn patch_hmo(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<i32>,
    Json(body): Json<PatchHmoRequest>,
) -> Result<Json<HmoResponse>, StatusCode> {
    // 1) Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        auth.claims.role_id,
        "hmo",
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
    let existing = hmo::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load hmo {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 3) Apply the patch
    let mut am: hmo::ActiveModel = existing.into();

    if let Some(v) = body.short_name {
        let v = v.trim().to_string();
        if v.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        am.short_name = Set(v);
    }

    if let Some(v) = body.long_name {
        let v = v.trim().to_string();
        if v.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        am.long_name = Set(v);
    }

    if let Some(v) = body.address {
        am.address = Set(v);
    }
    if let Some(v) = body.tax_account_number {
        am.tax_account_number = Set(v);
    }
    if let Some(v) = body.contact_nos {
        am.contact_nos = Set(v);
    }
    if let Some(v) = body.active {
        am.active = Set(v);
    }

    // Always update audit fields
    am.last_modified_by = Set(auth.claims.email.clone()); // <-- change if needed
    am.last_modified_on = Set(now_utc());

    // 4) Save
    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to update hmo {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated.into()))
}