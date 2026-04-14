use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait, Set, TransactionTrait};
use serde::{ Deserialize, Serialize};
use std::collections::BTreeMap;
use axum::extract::Path;
use chrono::Utc;
use rust_decimal::Decimal;
use crate::AppState;
use crate::{
    entities::{
    dental_service, dentist, endorsement, high_end_files, high_end_verification_information,
    hmo, master_list_member,
    verification, verification_status,
},
    handlers::AuthUser
};

#[derive(Debug, Serialize)]
pub struct HighEndFileResponse {
    pub id: i32,
    pub original_filename: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HighEndVerificationResponse {
    pub verification_id: i32,
    pub date_created: sea_orm::prelude::DateTimeWithTimeZone,
    pub status_id: i32,
    pub status_name: String,
    pub dentist_name: String,
    pub hmo_name: String,
    pub member_name: String,
    pub dental_service_name: String,
    pub files: Vec<HighEndFileResponse>,
}

#[derive(Debug, FromQueryResult)]
struct HighEndVerificationRow {
    pub verification_id: i32,
    pub date_created: sea_orm::prelude::DateTimeWithTimeZone,

    pub status_id: i32,
    pub status_name: String,

    pub dentist_last_name: String,
    pub dentist_given_name: String,
    pub dentist_middle_name: Option<String>,

    pub hmo_name: String,

    pub member_last_name: String,
    pub member_first_name: String,
    pub member_middle_name: String,

    pub dental_service_name: String,

    pub file_id: Option<i32>,
    pub original_filename: Option<String>,
    pub description: Option<String>,
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


// region: get_high_end_verifications
pub async fn get_high_end_verifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<HighEndVerificationResponse>>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    // ---- 1. Find all verifications whose dental_service.type_id=3
    let rows: Vec<HighEndVerificationRow> = verification::Entity::find()
        .join(JoinType::InnerJoin, verification::Relation::DentalService.def())
        // we use join_rev to join verification.status_id with verification_status.int_code
        .join_rev(
            JoinType::InnerJoin,
            verification_status::Entity::belongs_to(verification::Entity)
                .from (verification_status::Column::IntCode)
                .to(verification::Column::StatusId)
                .into()
        )
        .join(JoinType::InnerJoin, verification::Relation::Dentist.def())
        .join(JoinType::InnerJoin, verification::Relation::MasterListMember.def())
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::Endorsement.def(),
        )
        .join(
            JoinType::InnerJoin,
            endorsement::Relation::Hmo.def(),
        )
        .join(JoinType::LeftJoin, verification::Relation::HighEndFiles.def())
        .filter(dental_service::Column::TypeId.eq(3))
        .select_only()
        .column_as(verification::Column::Id, "verification_id")
        .column(verification::Column::DateCreated)
        .column_as(verification::Column::StatusId, "status_id")
        .column_as(verification_status::Column::Name, "status_name")
        .column_as(dentist::Column::LastName, "dentist_last_name")
        .column_as(dentist::Column::GivenName, "dentist_given_name")
        .column_as(dentist::Column::MiddleName, "dentist_middle_name")
        .column_as(hmo::Column::ShortName, "hmo_name")
        .column_as(master_list_member::Column::LastName, "member_last_name")
        .column_as(master_list_member::Column::FirstName, "member_first_name")
        .column_as(master_list_member::Column::MiddleName, "member_middle_name")
        .column_as(dental_service::Column::Name, "dental_service_name")
        .column_as(high_end_files::Column::Id, "file_id")
        .column_as(high_end_files::Column::OriginalFilename, "original_filename")
        .column_as(high_end_files::Column::Description, "description")
        .into_model::<HighEndVerificationRow>()
        .all(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut grouped: BTreeMap<i32, HighEndVerificationResponse> = BTreeMap::new();

    for row in rows {
        let entry = grouped
            .entry(row.verification_id)
            .or_insert_with(|| HighEndVerificationResponse {
                verification_id: row.verification_id,
                date_created: row.date_created,
                status_id: row.status_id,
                status_name: row.status_name.clone(),
                dentist_name: format_dentist_name(
                    &row.dentist_last_name,
                    &row.dentist_given_name,
                    row.dentist_middle_name.as_deref(),
                ),
                hmo_name: row.hmo_name.clone(),
                member_name: format_member_name(
                    &row.member_last_name,
                    &row.member_first_name,
                    &row.member_middle_name,
                ),
                dental_service_name: row.dental_service_name.clone(),
                files: vec![],
            });

        if let Some(file_id) = row.file_id {
            entry.files.push(HighEndFileResponse {
                id: file_id,
                original_filename: row.original_filename.clone(),
                description: row.description.clone(),
            });
        }
    }

    let response = grouped.into_values().collect::<Vec<_>>();

    Ok(Json(response))
}

// endregion: get_high_end_verifications

// region: post_high_end_verification_approval
#[derive(Debug, Deserialize)]
pub struct PostHighEndVerificationApprovalRequest {
    pub approved_cost: Decimal,
    pub dentist_notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostHighEndVerificationApprovalResponse {
    pub id: i32,
    pub verification_id: i32,
    pub approved_by: Option<String>,
    pub approved_cost: Option<Decimal>,
    pub approval_date: Option<chrono::NaiveDateTime>,
    pub dentist_notes: Option<String>,
    pub verification_status_id: i32,
}

pub async fn post_high_end_verification_approval(
    State(state): State<AppState>,
    Path(verification_id): Path<i32>,
    auth: AuthUser,
    Json(payload): Json<PostHighEndVerificationApprovalRequest>,
) -> Result<Json<PostHighEndVerificationApprovalResponse>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    // 1 --- Start a database transaction.
    let txn = db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ✅ Confirm the verification exists first.
    let existing_verification = verification::Entity::find_by_id(verification_id)
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(verification_model) = existing_verification else {
        return Err((StatusCode::NOT_FOUND, "Verification not found".to_string()));
    };

    // ✅ Current datetime for approval_date.
    let now = Utc::now().naive_utc();

    // ✅ Insert into high_end_verification_information.
    let inserted = high_end_verification_information::ActiveModel {
        verification_id: Set(verification_id),
        approved_by: Set(Some(auth.claims.email.clone())),
        approved_cost: Set(Some(payload.approved_cost)),
        approval_date: Set(Some(now)),
        dentist_notes: Set(payload.dentist_notes.clone()),
        ..Default::default()
    }
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ✅ Update verification.status_id to 3:"dentist quoted; waiting for approval code".
    let mut verification_am: verification::ActiveModel = verification_model.into();
    verification_am.status_id = Set(3);

    verification_am
        .update(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PostHighEndVerificationApprovalResponse {
        id: inserted.id,
        verification_id: inserted.verification_id,
        approved_by: inserted.approved_by,
        approved_cost: inserted.approved_cost,
        approval_date: inserted.approval_date,
        dentist_notes: inserted.dentist_notes,
        verification_status_id: 1,
    }))
}
// endregion: post_high_end_verification_approval
