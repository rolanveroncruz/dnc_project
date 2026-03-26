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
use crate::entities::{dentist_company_relations, endorsement_company};

#[derive(Debug, FromQueryResult)]
struct CompanyShortRow {
    pub id: i32,
    pub short_name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CompanyShort {
    pub id: i32,
    pub short_name: String,
}

// --- shared internal query
async fn get_companies_by_exclusive_flag(
    state: &AppState,
    dentist_id: i32,
    is_exclusive: bool,
) -> Result<Json<Vec<CompanyShort>>, StatusCode> {
    let rows: Vec<CompanyShortRow> = dentist_company_relations::Entity::find()
        // join dentist_company_relations -> endorsement_company
        .join(
            JoinType::InnerJoin,
            dentist_company_relations::Relation::EndorsementCompany.def(),
        )
        .filter(dentist_company_relations::Column::DentistId.eq(dentist_id))
        // Option<bool> column: only rows explicitly set to true/false
        .filter(dentist_company_relations::Column::IsExclusiveToCompany.eq(is_exclusive))
        // only select the columns we need
        .select_only()
        .column_as(endorsement_company::Column::Id, "id")
        .column_as(endorsement_company::Column::Name, "short_name")
        .into_model::<CompanyShortRow>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to fetch companies (id, short_name) for dentist_id={dentist_id} (exclusive={is_exclusive}): {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
        rows.into_iter()
            .map(|r| CompanyShort {
                id: r.id,
                short_name: r.short_name,
            })
            .collect(),
    ))
}

/// GET /dentists/:dentist_id/companies/exclusive
#[instrument(skip(state), err(Debug))]
pub async fn get_exclusive_to_companies_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<CompanyShort>>, StatusCode> {
    get_companies_by_exclusive_flag(&state, dentist_id, true).await
}

/// GET /dentists/:dentist_id/companies/not
#[instrument(skip(state), err(Debug))]
pub async fn get_not_companies_from_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<CompanyShort>>, StatusCode> {
    get_companies_by_exclusive_flag(&state, dentist_id, false).await
}

#[allow(dead_code)]
async fn ensure_company_exists(state: &AppState, company_id: i32) -> Result<(), StatusCode> {
    let exists = endorsement_company::Entity::find_by_id(company_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to check endorsement_company existence company_id={company_id}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}

#[allow(dead_code, unused)]
/// Insert or update (dentist_id, company_id) to have a desired exclusive flag.
/// - desired=true => exclusive-to
/// - desired=false => except-for
async fn upsert_relation_flag(
    state: &AppState,
    dentist_id: i32,
    company_id: i32,
    desired: bool,
) -> Result<StatusCode, StatusCode> {
    ensure_company_exists(state, company_id).await?;

    // Find the existing relation row (any flag)
    let existing = dentist_company_relations::Entity::find()
        .filter(dentist_company_relations::Column::DentistId.eq(dentist_id))
        .filter(dentist_company_relations::Column::CompanyId.eq(company_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to find dentist_company_relations dentist_id={dentist_id} company_id={company_id}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match existing {
        None => {
            // Create a new row
            let am = dentist_company_relations::ActiveModel {
                dentist_id: Set(dentist_id),
                company_id: Set(company_id),
                is_exclusive_to_company: Set(Some(desired)),
                ..Default::default()
            };

            am.insert(&state.db).await.map_err(|e| {
                tracing::error!(
                    "Failed to insert dentist_company_relations dentist_id={dentist_id} company_id={company_id} desired={desired}: {e:?}"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok(StatusCode::CREATED)
        }
        Some(model) => {
            // If already in the desired bucket => conflict
            if model.is_exclusive_to_company == Some(desired) {
                return Ok(StatusCode::CONFLICT);
            }

            // Otherwise, flip/move bucket via update
            let mut am: dentist_company_relations::ActiveModel = model.into();
            am.is_exclusive_to_company = Set(Some(desired));

            am.update(&state.db).await.map_err(|e| {
                tracing::error!(
                    "Failed to update dentist_company_relations dentist_id={dentist_id} company_id={company_id} desired={desired}: {e:?}"
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
    company_id: i32,
    expected: bool,
) -> Result<StatusCode, StatusCode> {
    // Find only rows in the expected bucket
    let model = dentist_company_relations::Entity::find()
        .filter(dentist_company_relations::Column::DentistId.eq(dentist_id))
        .filter(dentist_company_relations::Column::CompanyId.eq(company_id))
        .filter(dentist_company_relations::Column::IsExclusiveToCompany.eq(expected))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to find dentist_company_relations for delete dentist_id={dentist_id} company_id={company_id} expected={expected}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    model.delete(&state.db).await.map_err(|e| {
        tracing::error!(
            "Failed to delete dentist_company_relations dentist_id={dentist_id} company_id={company_id} expected={expected}: {e:?}"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /dentists/:dentist_id/companies/exclusive/:company_id
#[instrument(skip(state), err(Debug))]
pub async fn add_exclusive_to_company(
    State(state): State<AppState>,
    Path((dentist_id, company_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    upsert_relation_flag(&state, dentist_id, company_id, true).await
}

/// DELETE /dentists/:dentist_id/companies/exclusive/:company_id
#[instrument(skip(state), err(Debug))]
pub async fn remove_exclusive_to_company(
    State(state): State<AppState>,
    Path((dentist_id, company_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    delete_relation_flag(&state, dentist_id, company_id, true).await
}

/// POST /dentists/:dentist_id/companies/except/:company_id
#[instrument(skip(state), err(Debug))]
pub async fn add_except_for_company(
    State(state): State<AppState>,
    Path((dentist_id, company_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    upsert_relation_flag(&state, dentist_id, company_id, false).await
}

/// DELETE /dentists/:dentist_id/companies/except/:company_id
#[instrument(skip(state), err(Debug))]
pub async fn remove_except_for_company(
    State(state): State<AppState>,
    Path((dentist_id, company_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    delete_relation_flag(&state, dentist_id, company_id, false).await
}