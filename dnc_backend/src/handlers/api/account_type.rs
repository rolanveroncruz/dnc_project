use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::account_type;

#[instrument(skip(state), err(Debug))]
pub async fn get_all_account_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<account_type::Model>>, StatusCode> {
    let rows = account_type::Entity::find()
        .order_by_asc(account_type::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch account types: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
