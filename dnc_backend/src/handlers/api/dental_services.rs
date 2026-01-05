use axum::{extract::{Query, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ColumnTrait,  Condition, EntityTrait, FromQueryResult, JoinType, Order,
              PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::{dental_service, dental_service_type};
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct DentalServiceListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct DentalServiceRow {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub type_name: Option<String>, // LEFT JOIN => can be NULL
    pub last_modified_by: Option<String>,
    pub last_modified_on: chrono::DateTime<chrono::Utc>, // adjust type to your column type
}




#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_dental_services(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<DentalServiceListQuery>,
) -> Result<Json<PageResponse<DentalServiceRow>>, StatusCode>{
    // 1. Permission check
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dental_service",
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
    let mut query = dental_service::Entity::find()
        .join(JoinType::LeftJoin, dental_service::Relation::DentalServiceType.def())
        .filter(dental_service::Column::Active.eq(active));

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
            Condition::any().add(Expr::col(dental_service::Column::Name).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(dental_service::Column::Id, sort_order),
        "name" => query.order_by(dental_service::Column::Name, sort_order),
        "typeId" | "type_id" => query.order_by( dental_service::Column::TypeId,sort_order),
        "lastModifiedOn" | "last_modified_on" => query.order_by(dental_service::Column::LastModifiedOn, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // 5. Select only the columns we want + alias type name as 'type_name'
    let query = query
        .select_only()
        .column(dental_service::Column::Id)
        .column(dental_service::Column::Name)
        .column(dental_service::Column::Active)
        .column_as(dental_service_type::Column::Name, "type_name")
        .column_as(dental_service::Column::LastModifiedBy, "last_modified_by")
        .column_as(dental_service::Column::LastModifiedOn, "last_modified_on")
        .into_model::<DentalServiceRow>();

    // 5. Paginate
    let paginator = query.paginate(&state.db, page_size);
    let total_items = paginator
        .num_items()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items = paginator
        .fetch_page(page-1)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("user {} fetched page {} of {} dental services", user.claims.email, page, total_pages);

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))
}