use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{
     EntityTrait, FromQueryResult, JoinType, QuerySelect, RelationTrait,
};
use serde::Serialize;
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

#[derive(Debug, Serialize)]
pub struct VerificationLookupResponse {
    pub verification_id: i32,
    pub date: sea_orm::prelude::DateTimeWithTimeZone,
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
    pub date: sea_orm::prelude::DateTimeWithTimeZone,

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
            verification::Relation::VerificationStatus.def(),
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
        .column(verification::Column::StatusId)
        .column_as(verification_status::Column::Name, "status_name")
        .into_model::<VerificationLookupRow>()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = rows
        .into_iter()
        .map(|row| VerificationLookupResponse {
            verification_id: row.verification_id,
            date: row.date,
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