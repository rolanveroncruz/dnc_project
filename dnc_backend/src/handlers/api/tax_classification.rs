use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::tax_classification;

#[instrument(skip(state), err(Debug))]
pub async fn get_all_tax_classifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<tax_classification::Model>>, StatusCode> {
    let rows = tax_classification::Entity::find()
        .order_by_asc(tax_classification::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch tax classifications: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
