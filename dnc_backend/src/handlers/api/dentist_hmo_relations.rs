use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use tracing::instrument;

use crate::AppState;
use crate::entities::{dentist_hmo_relations, hmo};

#[derive(Debug, FromQueryResult)]
struct HmoShortNameRow {
    pub short_name: String,
}

// --- shared internal query
async fn get_hmo_short_names_by_exclusive_flag(
    state: &AppState,
    dentist_id: i32,
    is_exclusive: bool,
) -> Result<Json<Vec<String>>, StatusCode> {
    let rows: Vec<HmoShortNameRow> = dentist_hmo_relations::Entity::find()
        // join dentist_hmo_relations -> hmo
        .join(
            JoinType::InnerJoin,
            dentist_hmo_relations::Relation::Hmo.def(),
        )
        .filter(dentist_hmo_relations::Column::DentistId.eq(dentist_id))
        // Option<bool> column: only rows explicitly set to true/false
        .filter(dentist_hmo_relations::Column::IsExclusiveToHmo.eq(is_exclusive))
        // only select the one column we need
        .select_only()
        .column_as(hmo::Column::ShortName, "short_name")
        .into_model::<HmoShortNameRow>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch HMO short_names for dentist_id={dentist_id} (exclusive={is_exclusive}): {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows.into_iter().map(|r| r.short_name).collect()))
}

/// GET /dentists/:dentist_id/hmos/exclusive
#[instrument(skip(state), err(Debug))]
pub async fn get_exclusive_to_hmos_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<String>>, StatusCode> {
    get_hmo_short_names_by_exclusive_flag(&state, dentist_id, true).await
}

/// GET /dentists/:dentist_id/hmos/not
#[instrument(skip(state), err(Debug))]
pub async fn get_not_hmos_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<String>>, StatusCode> {
    get_hmo_short_names_by_exclusive_flag(&state, dentist_id, false).await
}
