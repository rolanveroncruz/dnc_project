use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, ModelTrait, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use tracing::instrument;

use crate::AppState;
use crate::entities::{dentist_hmo_relations, hmo};

#[derive(Debug, FromQueryResult)]
struct HmoShortRow {
    pub id: i32,
    pub short_name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct HmoShort {
    pub id: i32,
    pub short_name: String,
}

// --- shared internal query
async fn get_hmos_by_exclusive_flag(
    state: &AppState,
    dentist_id: i32,
    is_exclusive: bool,
) -> Result<Json<Vec<HmoShort>>, StatusCode> {
    let rows: Vec<HmoShortRow> = dentist_hmo_relations::Entity::find()
        // join dentist_hmo_relations -> hmo
        .join(
            JoinType::InnerJoin,
            dentist_hmo_relations::Relation::Hmo.def(),
        )
        .filter(dentist_hmo_relations::Column::DentistId.eq(dentist_id))
        // Option<bool> column: only rows explicitly set to true/false
        .filter(dentist_hmo_relations::Column::IsExclusiveToHmo.eq(is_exclusive))
        // only select the columns we need
        .select_only()
        .column_as(hmo::Column::Id, "id")
        .column_as(hmo::Column::ShortName, "short_name")
        .into_model::<HmoShortRow>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to fetch HMOs (id, short_name) for dentist_id={dentist_id} (exclusive={is_exclusive}): {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| HmoShort {
                id: r.id,
                short_name: r.short_name,
            })
            .collect(),
    ))
}

/// GET /dentists/:dentist_id/hmos/exclusive
#[instrument(skip(state), err(Debug))]
pub async fn get_exclusive_to_hmos_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<HmoShort>>, StatusCode> {
    get_hmos_by_exclusive_flag(&state, dentist_id, true).await
}

/// GET /dentists/:dentist_id/hmos/not
#[instrument(skip(state), err(Debug))]
pub async fn get_not_hmos_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<HmoShort>>, StatusCode> {
    get_hmos_by_exclusive_flag(&state, dentist_id, false).await
}

#[allow(dead_code)]
async fn ensure_hmo_exists(state: &AppState, hmo_id: i32) -> Result<(), StatusCode> {
    let exists = hmo::Entity::find_by_id(hmo_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check HMO existence hmo_id={hmo_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(())
}

#[allow(dead_code, unused)]
/// Insert or update (dentist_id, hmo_id) to have a desired exclusive flag.
/// - desired=true => exclusive-to
/// - desired=false => except-for
async fn upsert_relation_flag(
    state: &AppState,
    dentist_id: i32,
    hmo_id: i32,
    desired: bool,
) -> Result<StatusCode, StatusCode> {
    ensure_hmo_exists(state, hmo_id).await?;

    // Find the existing relation row (any flag)
    let existing = dentist_hmo_relations::Entity::find()
        .filter(dentist_hmo_relations::Column::DentistId.eq(dentist_id))
        .filter(dentist_hmo_relations::Column::HmoId.eq(hmo_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to find dentist_hmo_relations dentist_id={dentist_id} hmo_id={hmo_id}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match existing {
        None => {
            // Create a new row
            let am = dentist_hmo_relations::ActiveModel {
                dentist_id: Set(dentist_id),
                hmo_id: Set(hmo_id),
                is_exclusive_to_hmo: Set(Some(desired)),
                ..Default::default()
            };

            am.insert(&state.db).await.map_err(|e| {
                tracing::error!(
                    "Failed to insert dentist_hmo_relations dentist_id={dentist_id} hmo_id={hmo_id} desired={desired}: {e:?}"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok(StatusCode::CREATED)
        }
        Some(model) => {
            // If already in the desired bucket => conflict
            if model.is_exclusive_to_hmo == Some(desired) {
                return Ok(StatusCode::CONFLICT);
            }

            // Otherwise, flip/move bucket via update
            let mut am: dentist_hmo_relations::ActiveModel = model.into();
            am.is_exclusive_to_hmo = Set(Some(desired));

            am.update(&state.db).await.map_err(|e| {
                tracing::error!(
                    "Failed to update dentist_hmo_relations dentist_id={dentist_id} hmo_id={hmo_id} desired={desired}: {e:?}"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok(StatusCode::OK)
        }
    }
}

#[allow(dead_code, unused)]
/// Delete the relation only if it matches the expected flag.
/// - expected=true => remove exclusive-to
/// - expected=false => remove except-for
async fn delete_relation_flag(
    state: &AppState,
    dentist_id: i32,
    hmo_id: i32,
    expected: bool,
) -> Result<StatusCode, StatusCode> {
    // Find only rows in the expected bucket
    let model = dentist_hmo_relations::Entity::find()
        .filter(dentist_hmo_relations::Column::DentistId.eq(dentist_id))
        .filter(dentist_hmo_relations::Column::HmoId.eq(hmo_id))
        .filter(dentist_hmo_relations::Column::IsExclusiveToHmo.eq(expected))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to find dentist_hmo_relations for delete dentist_id={dentist_id} hmo_id={hmo_id} expected={expected}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    model.delete(&state.db).await.map_err(|e| {
        tracing::error!(
            "Failed to delete dentist_hmo_relations dentist_id={dentist_id} hmo_id={hmo_id} expected={expected}: {e:?}"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /dentists/:dentist_id/hmos/exclusive/:hmo_id
#[instrument(skip(state), err(Debug))]
pub async fn add_exclusive_to_hmo(
    State(state): State<AppState>,
    Path((dentist_id, hmo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    upsert_relation_flag(&state, dentist_id, hmo_id, true).await
}

/// DELETE /dentists/:dentist_id/hmos/exclusive/:hmo_id
#[instrument(skip(state), err(Debug))]
pub async fn remove_exclusive_to_hmo(
    State(state): State<AppState>,
    Path((dentist_id, hmo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    delete_relation_flag(&state, dentist_id, hmo_id, true).await
}

/// POST /dentists/:dentist_id/hmos/except/:hmo_id
#[instrument(skip(state), err(Debug))]
pub async fn add_except_for_hmo(
    State(state): State<AppState>,
    Path((dentist_id, hmo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    upsert_relation_flag(&state, dentist_id, hmo_id, false).await
}

/// DELETE /dentists/:dentist_id/hmos/except/:hmo_id
#[instrument(skip(state), err(Debug))]
pub async fn remove_except_for_hmo(
    State(state): State<AppState>,
    Path((dentist_id, hmo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    delete_relation_flag(&state, dentist_id, hmo_id, false).await
}
