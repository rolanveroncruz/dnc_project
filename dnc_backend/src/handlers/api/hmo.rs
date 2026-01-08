use axum::{extract::{Query, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ColumnTrait, Condition, EntityTrait, FromQueryResult, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
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
    pub long_name: Option<String>,
    pub address: Option<String>,
    pub tax_account_number: Option<String>,
    pub contact_nos: Option<String>,
    pub active: bool,
    pub last_endorsement_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_collection_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_modified_by: String,
    pub last_modified_on: chrono::DateTime<chrono::Utc>, // adjust type to your column type
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
        .column_as(Expr::cust("NULL"), "last_endorsement_date")
        .column_as(Expr::cust("NULL"), "last_collection_date")
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
