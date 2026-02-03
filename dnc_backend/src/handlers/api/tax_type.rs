use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::tax_type;

#[instrument(skip(state), err(Debug))]
pub async fn get_all_tax_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<tax_type::Model>>, StatusCode> {
    let rows = tax_type::Entity::find()
        .order_by_asc(tax_type::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch tax types: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
