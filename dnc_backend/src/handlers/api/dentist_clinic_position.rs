use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::position;

#[instrument(skip(state))]
pub async fn get_dentist_clinic_positions(
    State(state): State<AppState>,
) -> Result<Json<Vec<position::Model>>, StatusCode> {
    position::Entity::find()
        .order_by_asc(position::Column::Id)
        .all(&state.db)
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!("Failed to fetch positions: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
