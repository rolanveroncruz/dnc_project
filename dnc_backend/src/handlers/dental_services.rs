use axum::{extract::{Query, State}, http::StatusCode, Json};
use crate::AppState;
use crate::handlers::structs::AuthUser;
use sea_orm::{ColumnTrait,  Condition, EntityTrait, Order, PaginatorTrait,QueryFilter, QueryOrder};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::Deserialize;
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::dental_service;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;
use crate::handlers::{ListQuery, PageResponse};
use sea_orm::sea_query::Expr;

#[derive(Debug, Deserialize)]
pub struct DentalServiceListQuery {
    #[serde(flatten)]
    pub base: ListQuery,
}

pub async fn get_dental_services(
    State(state): State<AppState>,
    user:AuthUser,
    Query(params): Query<DentalServiceListQuery>,
) -> Result<Json<PageResponse<dental_service::Model>>, StatusCode>{
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
    }

    // 2. Defaults + basic validation/clamping
    let page = params.base.page.unwrap_or(1).max(1);
    let page_size = params.base.page_size.unwrap_or(25).clamp(1, 200);
    let active = params.base.active.unwrap_or(true);

    let sort = params.base.sort.as_deref().unwrap_or("name");
    println!("sort: {}", sort);
    let order = params.base.order.as_deref().unwrap_or("asc");
    println!("order: {}", order);

    // 3. Build the query
    let mut query = dental_service::Entity::find()
        .filter(dental_service::Column::Active.eq(active));

    // q=clean (search by name; using ILIKE for Postgres-ish case-insensitive search)
    if let Some(q) = params.base.q.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        let pattern = format!("%{}%", q);
        query = query.filter(
            Condition::any().add(Expr::col(dental_service::Column::Name).ilike(pattern)),);
    }

    // 4. Safe sort mapping (never trust raw column names from the client!)
    let sort_col = match sort {
        "id" => dental_service::Column::Id,
        "name" => dental_service::Column::Name,
        "typeId" | "type_id" => dental_service::Column::TypeId,
        "lastModifiedOn" | "last_modified_on" => dental_service::Column::LastModifiedOn,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    let sort_order = match order {
        "asc" => Order::Asc,
        "desc" => Order::Desc,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    query = query.order_by(sort_col, sort_order);

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

    Ok(Json(PageResponse{
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }))



}