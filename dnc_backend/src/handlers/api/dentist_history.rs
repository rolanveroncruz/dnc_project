use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::dentist_history;

#[instrument(skip(state), err(Debug))]
pub async fn get_all_dentist_histories(
    State(state): State<AppState>,
) -> Result<Json<Vec<dentist_history::Model>>, StatusCode> {
    let rows = dentist_history::Entity::find()
        .order_by_asc(dentist_history::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist histories: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
