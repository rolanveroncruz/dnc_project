use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, ModelTrait, QueryFilter, QuerySelect, RelationTrait, Set};
use sea_orm::prelude::Expr;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::{dentist, dentist_clinic, dental_clinic, position};

/// Joined row returned to the client
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistClinicWithNames {
    pub dentist_id: i32,
    pub clinic_id: Option<i32>,

    pub position_id: Option<i32>,
    pub schedule: Option<String>,

    pub last_name: String,
    pub given_name: String,
    pub middle_name: Option<String>,

    pub clinic_name: Option<String>,
    pub position_name: Option<String>,
}

/// Base select used by all endpoints
fn dentist_clinic_select_base() -> sea_orm::Select<dentist_clinic::Entity> {
    dentist_clinic::Entity::find()
        // dentist_clinic -> dentist (required)
        .join(JoinType::InnerJoin, dentist_clinic::Relation::Dentist.def())
        // dentist_clinic -> dental_clinic (optional because clinic_id is Option<i32>)
        .join(JoinType::LeftJoin, dentist_clinic::Relation::DentalClinic.def())
        // ANNOTATED CHANGE: join dentist_clinic -> position (optional because position_id is Option<i32>)
        .join(JoinType::LeftJoin, dentist_clinic::Relation::Position.def())
        .select_only()
        // dentist_clinic fields
        .column_as(dentist_clinic::Column::DentistId, "dentist_id")
        .column_as(dentist_clinic::Column::ClinicId, "clinic_id")
        .column_as(dentist_clinic::Column::PositionId, "position_id")
        .column_as(dentist_clinic::Column::Schedule, "schedule")
        // dentist name fields
        .expr_as(Expr::col((dentist::Entity, dentist::Column::LastName)), "last_name")
        .expr_as(Expr::col((dentist::Entity, dentist::Column::GivenName)), "given_name")
        .expr_as(Expr::col((dentist::Entity, dentist::Column::MiddleName)), "middle_name")
        // clinic name field (nullable because clinic left-join)
        .expr_as(Expr::col((dental_clinic::Entity, dental_clinic::Column::Name)), "clinic_name")
        .expr_as(Expr::col((position::Entity, position::Column::Name)), "position_name")
}

/// GET /dentist_clinics
#[instrument(skip(state), err(Debug))]
pub async fn get_all_dentist_clinics(
    State(state): State<AppState>,
) -> Result<Json<Vec<DentistClinicWithNames>>, StatusCode> {
    let rows = dentist_clinic_select_base()
        .into_model::<DentistClinicWithNames>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist_clinics: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}

/// GET /dentists/{dentist_id}/clinics
#[instrument(skip(state), err(Debug))]
pub async fn get_clinics_for_dentist_id(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
) -> Result<Json<Vec<DentistClinicWithNames>>, StatusCode> {
    let rows = dentist_clinic_select_base()
        .filter(dentist_clinic::Column::DentistId.eq(dentist_id))
        .into_model::<DentistClinicWithNames>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch clinics for dentist_id={dentist_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}

/// GET /clinics/{clinic_id}/dentists
#[instrument(skip(state), err(Debug))]
pub async fn get_dentists_for_clinic_id(
    State(state): State<AppState>,
    Path(clinic_id): Path<i32>,
) -> Result<Json<Vec<DentistClinicWithNames>>, StatusCode> {
    let rows = dentist_clinic_select_base()
        .filter(dentist_clinic::Column::ClinicId.eq(clinic_id))
        .into_model::<DentistClinicWithNames>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentists for clinic_id={clinic_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}


// -------------------------
// Request payloads
// -------------------------

#[derive(Debug, Deserialize)]
pub struct AddDentistClinicRequest {
    pub clinic_id: i32,
    pub position_id: Option<i32>,
    pub schedule: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct AddRemoveResult<T> {
    pub data: T,
}

// -------------------------
// POST /dentists/{dentist_id}/clinics
// -------------------------
#[instrument(skip(state, payload), err(Debug))]
pub async fn add_dentist_clinic(
    State(state): State<AppState>,
    Path(dentist_id): Path<i32>,
    Json(payload): Json<AddDentistClinicRequest>,
) -> Result<(StatusCode, Json<DentistClinicWithNames>), StatusCode> {
    // 1) Ensure dentist exists
    let dentist_exists = dentist::Entity::find_by_id(dentist_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check dentist existence dentist_id={dentist_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !dentist_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // 2) Ensure clinic exists
    let clinic_exists = dental_clinic::Entity::find_by_id(payload.clinic_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to check clinic existence clinic_id={}: {e:?}",
                payload.clinic_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !clinic_exists {
        return Err(StatusCode::NOT_FOUND);
    }
    if let Some(pos_id)=payload.position_id{
        let pos_exists = position::Entity::find_by_id(pos_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check position existence position_id={pos_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .is_some();

        if !pos_exists {
            return Err(StatusCode::NOT_FOUND);
        }
    }

    // 3) Prevent duplicate link (app-level)
    let already_linked = dentist_clinic::Entity::find()
        .filter(dentist_clinic::Column::DentistId.eq(dentist_id))
        .filter(dentist_clinic::Column::ClinicId.eq(payload.clinic_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to check existing dentist_clinic link dentist_id={dentist_id} clinic_id={}: {e:?}",
                payload.clinic_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if already_linked {
        // Conflict is a good signal for "already exists"
        return Err(StatusCode::CONFLICT);
    }

    // 4) Insert
    let am = dentist_clinic::ActiveModel {
        dentist_id: Set(dentist_id),
        clinic_id: Set(Some(payload.clinic_id)),
        position_id: Set(payload.position_id),
        schedule: Set(payload.schedule),
        ..Default::default()
    };

    let _inserted = am.insert(&state.db).await.map_err(|e| {
        tracing::error!(
            "Failed to insert dentist_clinic dentist_id={dentist_id} clinic_id={}: {e:?}",
            payload.clinic_id
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 5) Return the created row in your joined/view shape
    // (Fetch by dentist+clinic; safest since id is DB-generated)
    let row = dentist_clinic_select_base()
        .filter(dentist_clinic::Column::DentistId.eq(dentist_id))
        .filter(dentist_clinic::Column::ClinicId.eq(payload.clinic_id))
        .into_model::<DentistClinicWithNames>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to fetch inserted dentist_clinic dentist_id={dentist_id} clinic_id={}: {e:?}",
                payload.clinic_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(row)))
}

// -------------------------
// DELETE /dentists/{dentist_id}/clinics/{clinic_id}
// -------------------------
#[instrument(skip(state), err(Debug))]
pub async fn remove_dentist_clinic(
    State(state): State<AppState>,
    Path((dentist_id, clinic_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    // Find the linking row (dentist_id + clinic_id)
    let model = dentist_clinic::Entity::find()
        .filter(dentist_clinic::Column::DentistId.eq(dentist_id))
        .filter(dentist_clinic::Column::ClinicId.eq(clinic_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to find dentist_clinic for delete dentist_id={dentist_id} clinic_id={clinic_id}: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Delete it
    model.delete(&state.db).await.map_err(|e| {
        tracing::error!(
            "Failed to delete dentist_clinic dentist_id={dentist_id} clinic_id={clinic_id}: {e:?}"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}