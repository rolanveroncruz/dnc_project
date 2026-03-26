use axum::{extract::State, http::StatusCode, Json};
use axum::extract::Path;
use sea_orm::{ActiveModelTrait, EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait, Set};
use serde::{Serialize, Deserialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        dental_service,
        dentist,
        master_list_member,
        verification,
        verification_status,
    },
};
use crate::handlers::AuthUser;

#[derive(Debug, Serialize)]
pub struct VerificationLookupResponse {
    pub verification_id: i32,
    pub date_created: sea_orm::prelude::DateTimeWithTimeZone,
    pub dentist_id: i32,
    pub dentist_name: String,
    pub master_list_member_id: i32,
    pub master_list_member_name: String,
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub status_id: i32,
    pub status_name: String,
}

#[derive(Debug, FromQueryResult)]
struct VerificationLookupRow {
    pub verification_id: i32,
    pub date_created: sea_orm::prelude::DateTimeWithTimeZone,

    pub dentist_id: i32,
    pub dentist_last_name: String,
    pub dentist_given_name: String,
    pub dentist_middle_name: Option<String>,

    pub master_list_member_id: i32,
    pub member_last_name: String,
    pub member_first_name: String,
    pub member_middle_name: String,

    pub dental_service_id: i32,
    pub dental_service_name: String,

    pub status_id: i32,
    pub status_name: String,
}

fn format_dentist_name(
    last_name: &str,
    given_name: &str,
    middle_name: Option<&str>,
) -> String {
    match middle_name {
        Some(m) if !m.trim().is_empty() => format!("{}, {} {}", last_name, given_name, m),
        _ => format!("{}, {}", last_name, given_name),
    }
}

fn format_member_name(
    last_name: &str,
    first_name: &str,
    middle_name: &str,
) -> String {
    if middle_name.trim().is_empty() {
        format!("{}, {}", last_name, first_name)
    } else {
        format!("{}, {} {}", last_name, first_name, middle_name)
    }
}

#[instrument(skip(state))]
pub async fn get_all_verifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<VerificationLookupResponse>>, StatusCode> {
    let rows: Vec<VerificationLookupRow> = verification::Entity::find()
        .join(
            JoinType::InnerJoin,
            verification::Relation::Dentist.def(),
        )
        .join(
            JoinType::InnerJoin,
            verification::Relation::MasterListMember.def(),
        )
        .join(
            JoinType::InnerJoin,
            verification::Relation::DentalService.def(),
        )
        .join(
            JoinType::InnerJoin,
            verification::Entity::belongs_to(verification_status::Entity)
                .from(verification::Column::StatusId)
                .to(verification_status::Column::IntCode)
                .into(),
        )
        .select_only()
        .column_as(verification::Column::Id, "verification_id")
        .column_as(verification::Column::DateCreated, "date_created")
        .column(verification::Column::DentistId)
        .column_as(dentist::Column::LastName, "dentist_last_name")
        .column_as(dentist::Column::GivenName, "dentist_given_name")
        .column_as(dentist::Column::MiddleName, "dentist_middle_name")
        .column_as(verification::Column::MemberId, "master_list_member_id")
        .column_as(master_list_member::Column::LastName, "member_last_name")
        .column_as(master_list_member::Column::FirstName, "member_first_name")
        .column_as(master_list_member::Column::MiddleName, "member_middle_name")
        .column_as(verification::Column::DentalServiceId, "dental_service_id")
        .column_as(dental_service::Column::Name, "dental_service_name")
        .column_as(verification_status::Column::IntCode, "status_id")
        .column_as(verification_status::Column::Name, "status_name")
        .into_model::<VerificationLookupRow>()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = rows
        .into_iter()
        .map(|row| VerificationLookupResponse {
            verification_id: row.verification_id,
            date_created: row.date_created,
            dentist_id: row.dentist_id,
            dentist_name: format_dentist_name(
                &row.dentist_last_name,
                &row.dentist_given_name,
                row.dentist_middle_name.as_deref(),
            ),
            master_list_member_id: row.master_list_member_id,
            master_list_member_name: format_member_name(
                &row.member_last_name,
                &row.member_first_name,
                &row.member_middle_name,
            ),
            dental_service_id: row.dental_service_id,
            dental_service_name: row.dental_service_name,
            status_id: row.status_id,
            status_name: row.status_name,
        })
        .collect::<Vec<_>>();

    Ok(Json(response))
}



// region: Create Verification
#[derive(Debug, Deserialize)]
pub struct CreateVerificationRequest {
    pub dentist_id: i32,
    pub member_id: i32,
    pub dental_service_id: i32,
}
#[derive(Debug, Serialize)]
pub struct CreateVerificationResponse {
    pub id: i32,
    pub date_created: sea_orm::prelude::DateTimeWithTimeZone,
    pub created_by: String,
    pub dentist_id: i32,
    pub member_id: i32,
    pub dental_service_id: i32,
    pub date_service_performed: Option<sea_orm::prelude::Date>,
    pub status_id: i32,
    pub approved_by: Option<String>,
    pub approval_date: Option<sea_orm::prelude::DateTimeWithTimeZone>,
    pub approval_code: Option<String>,
}

#[instrument(skip(state), err(Debug))]
pub async fn create_verification(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateVerificationRequest>,
) -> Result<(StatusCode, Json<CreateVerificationResponse>), (StatusCode, String)> {
    let now = chrono::Utc::now().fixed_offset();

    // find the dental service record
    let dental_service = dental_service::Entity::find_by_id(payload.dental_service_id)
        .one(&state.db)
    .await
        .map_err(internal_error)?
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Dental service not found".to_string()))?;

    // status_id=2 if dental_service.type_id==3, else 1
    let status_id = if dental_service.type_id==3 {2} else {1};

    let new_verification = verification::ActiveModel {
        date_created: Set(now),
        created_by: Set(auth_user.claims.email),
        dentist_id: Set(payload.dentist_id),
        member_id: Set(payload.member_id),
        dental_service_id: Set(payload.dental_service_id),
        date_service_performed: Set(None),
        status_id: Set(status_id), // temporary
        approved_by: Set(None),
        approval_date: Set(None),
        approval_code: Set(None),
        ..Default::default()
    };

    let inserted = new_verification
        .insert(&state.db)
        .await
        .map_err(internal_error)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateVerificationResponse {
            id: inserted.id,
            date_created: inserted.date_created,
            created_by: inserted.created_by,
            dentist_id: inserted.dentist_id,
            member_id: inserted.member_id,
            dental_service_id: inserted.dental_service_id,
            date_service_performed: inserted.date_service_performed,
            status_id: inserted.status_id,
            approved_by: inserted.approved_by,
            approval_date: inserted.approval_date,
            approval_code: inserted.approval_code,
        }),
    ))
}

fn internal_error(err: sea_orm::DbErr) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// endregion: Create Verification


// region:Cancel Verification
#[instrument(skip(state), err(Debug))]
pub async fn cancel_verification(
    State(state): State<AppState>,
    Path(verification_id): Path<i32>,
) -> Result<Json<verification::Model>, (StatusCode, String)> {
    tracing::info!("cancel_verification({})", verification_id);
    let verification = verification::Entity::find_by_id(verification_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch verification: {e}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Verification with id {verification_id} not found"),
            )
        })?;

    let mut verification_active: verification::ActiveModel = verification.into();
    verification_active.status_id = Set(0);

    let updated = verification_active
        .update(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to cancel verification: {e}"),
            )
        })?;

    Ok(Json(updated))
}
// endregion: Cancel Verification
