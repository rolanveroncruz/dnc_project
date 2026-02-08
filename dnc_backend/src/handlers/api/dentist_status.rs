use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::AppState;
use crate::entities::dentist_status;

#[instrument(skip(state), err(Debug))]
pub async fn get_all_dentist_status(
    State(state): State<AppState>,
) -> Result<Json<Vec<dentist_status::Model>>, StatusCode> {
    let rows = dentist_status::Entity::find()
        .order_by_asc(dentist_status::Column::Name)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist status: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}
