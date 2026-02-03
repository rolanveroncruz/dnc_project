use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use sea_orm::prelude::Expr;
use serde::Serialize;
use tracing::instrument;

use crate::AppState;
use crate::entities::{dentist, dentist_clinic, dental_clinic};

/// Joined row returned to the client
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistClinicWithNames {
    pub dentist_id: i32,
    pub clinic_id: Option<i32>,

    pub position: Option<String>,
    pub schedule: Option<String>,

    pub last_name: String,
    pub given_name: String,
    pub middle_name: Option<String>,

    pub clinic_name: Option<String>,
}

/// Base select used by all endpoints
fn dentist_clinic_select_base() -> sea_orm::Select<dentist_clinic::Entity> {
    dentist_clinic::Entity::find()
        // dentist_clinic -> dentist (required)
        .join(JoinType::InnerJoin, dentist_clinic::Relation::Dentist.def())
        // dentist_clinic -> dental_clinic (optional because clinic_id is Option<i32>)
        .join(JoinType::LeftJoin, dentist_clinic::Relation::DentalClinic.def())
        .select_only()
        // dentist_clinic fields
        .column_as(dentist_clinic::Column::DentistId, "dentist_id")
        .column_as(dentist_clinic::Column::ClinicId, "clinic_id")
        .column_as(dentist_clinic::Column::Position, "position")
        .column_as(dentist_clinic::Column::Schedule, "schedule")
        // dentist name fields
        .expr_as(Expr::col((dentist::Entity, dentist::Column::LastName)), "last_name")
        .expr_as(Expr::col((dentist::Entity, dentist::Column::GivenName)), "given_name")
        .expr_as(Expr::col((dentist::Entity, dentist::Column::MiddleName)), "middle_name")
        // clinic name field (nullable because clinic left-join)
        .expr_as(Expr::col((dental_clinic::Entity, dental_clinic::Column::Name)), "clinic_name")
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
