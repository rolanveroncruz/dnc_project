use axum::{extract::{Query, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ Condition, EntityTrait, FromQueryResult,  Order,
              PaginatorTrait, QueryFilter, QueryOrder, QuerySelect };
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Serialize, Deserialize};
use crate::entities::{dental_service_type};
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct DentalServiceTypeListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct DentalServiceTypeRow {
    pub id: i32,
    pub name: String,
}




#[instrument(
    skip(state),
    err(Debug)
)]
pub async fn get_dental_service_types(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<DentalServiceTypeListQuery>,
) -> Result<Json<PageResponse<DentalServiceTypeRow>>, StatusCode>{
    // 1. Permission check
    if user.claims.role_id != 1 {
        return Err(StatusCode::FORBIDDEN);
    }

    // 2. Defaults + basic validation/clamping
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);

    let sort = params.base.sort.as_deref().unwrap_or("name");
    println!("sort: {}", sort);
    let order = params.base.order.as_deref().unwrap_or("asc");
    println!("order: {}", order);

    // 3. Build the query (JOIN to dental_service_type)
    let mut query = dental_service_type::Entity::find();

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
            Condition::any().add(Expr::col(dental_service_type::Column::Name).ilike(pattern)),);
    }

    query = match sort {
        "id" => query.order_by(dental_service_type::Column::Id, sort_order),
        "name" => query.order_by(dental_service_type::Column::Name, sort_order),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // 5. Select only the columns we want + alias type name as 'type_name'
    let query = query
        .select_only()
        .column(dental_service_type::Column::Id)
        .column(dental_service_type::Column::Name)
        .into_model::<DentalServiceTypeRow>();

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
