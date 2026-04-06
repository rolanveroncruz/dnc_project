use axum::{extract::State, http::StatusCode, Json};
use axum::extract::Path;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
use serde::{Serialize, Deserialize};
use tracing::instrument;
use chrono::{ Utc};
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
    pub record_tooth: bool,
    pub status_id: i32,
    pub status_name: String,
    pub approval_code: Option<String>,
    pub approved_by: Option<String>,
    pub approval_date: Option<sea_orm::prelude::DateTimeWithTimeZone>,
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
    pub record_tooth: bool,

    pub status_id: i32,
    pub status_name: String,

    pub approval_code: Option<String>,
    pub approved_by: Option<String>,
    pub approval_date: Option<sea_orm::prelude::DateTimeWithTimeZone>,
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
        .column_as(dental_service::Column::RecordTooth, "record_tooth")
        .column_as(verification_status::Column::IntCode, "status_id")
        .column_as(verification_status::Column::Name, "status_name")
        .column_as(verification::Column::ApprovalCode, "approval_code")
        .column_as(verification::Column::ApprovedBy, "approved_by")
        .column_as(verification::Column::ApprovalDate, "approval_date")
        .order_by_desc(verification::Column::DateCreated)
        .into_model::<VerificationLookupRow>()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = rows
        .into_iter()
        .map(|row| {
            let (approval_code, approved_by, approval_date) = if row.status_id==99 {
                (row.approval_code, row.approved_by, row.approval_date)
            } else{
                (None, None, None)
            };
            VerificationLookupResponse {
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
                record_tooth: row.record_tooth,
                status_id: row.status_id,
                status_name: row.status_name,
                approval_code,
                approved_by,
                approval_date,
            }
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
    let now = Utc::now().fixed_offset();

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


// region: Get Approval Code
#[derive(Debug, Deserialize)]
pub struct GetApprovalCodeRequest {
    pub date_service_performed: sea_orm::prelude::Date,
}

#[derive(Debug, Serialize)]
pub struct GetApprovalCodeResponse {
    pub approval_code: String,
}

/// Temporary approval code generator.
/// Replace this later with your real business rule.

#[instrument(skip(state, auth_user, payload), err(Debug))]
pub async fn get_approval_code_for_verification_id(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(verification_id): Path<i32>,
    Json(payload): Json<GetApprovalCodeRequest>,
) -> Result<Json<GetApprovalCodeResponse>, (StatusCode, String)> {
    let verification_model = verification::Entity::find()
        .filter(verification::Column::Id.eq(verification_id))
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Verification {} not found", verification_id),
            )
        })?;

    let approval_code = generate_approval_code(verification_id);

    let mut verification_active: verification::ActiveModel = verification_model.into();
    verification_active.date_service_performed = Set(Some(payload.date_service_performed));
    verification_active.approved_by = Set(Some(auth_user.claims.email.clone()));
    verification_active.approval_date = Set(Some(Utc::now().into()));
    verification_active.approval_code = Set(Some(approval_code.clone()));
    verification_active.status_id = Set(99);

    verification_active
        .update(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(GetApprovalCodeResponse { approval_code }))
}

// endregion: Get Approval Code


// region: generate_approval_code

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const APPROVAL_CODE_ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const ID_PART_LEN: usize = 7;
const TAG_PART_LEN: usize = 5;

fn generate_approval_code(verification_id: i32) -> String {
    assert!(verification_id >= 0, "verification_id must be non-negative");

    let secret = "Dental Network Company's Random Secret";

    let id_u64 = verification_id as u64;

    let mut mask_mac =
        <HmacSha256 as KeyInit>::new_from_slice(secret.as_bytes())
            .expect("invalid HMAC key");
    mask_mac.update(b"approval-code-mask:v1");
    let mask_bytes = mask_mac.finalize().into_bytes();

    let mut mask_arr = [0u8; 8];
    mask_arr.copy_from_slice(&mask_bytes[..8]);
    let mask_35 = u64::from_be_bytes(mask_arr) & ((1u64 << 35) - 1);

    let obfuscated_id = id_u64 ^ mask_35;

    let mut tag_mac =
        <HmacSha256 as KeyInit>::new_from_slice(secret.as_bytes())
            .expect("invalid HMAC key");
    tag_mac.update(b"approval-code-tag:v1:");
    tag_mac.update(verification_id.to_string().as_bytes());
    let tag_bytes = tag_mac.finalize().into_bytes();

    let mut tag_arr = [0u8; 4];
    tag_arr.copy_from_slice(&tag_bytes[..4]);
    let tag_25 = (u32::from_be_bytes(tag_arr) >> 7) & ((1u32 << 25) - 1);

    let id_part = encode_base32_fixed(obfuscated_id, ID_PART_LEN);
    let tag_part = encode_base32_fixed(tag_25 as u64, TAG_PART_LEN);

    let raw = format!("{id_part}{tag_part}");
    format_approval_code(&raw)
}

fn encode_base32_fixed(mut value: u64, len: usize) -> String {
    let mut out = vec!['A'; len];

    for i in (0..len).rev() {
        let idx = (value & 0b1_1111) as usize;
        out[i] = APPROVAL_CODE_ALPHABET[idx] as char;
        value >>= 5;
    }

    out.into_iter().collect()
}

fn format_approval_code(raw: &str) -> String {
    debug_assert_eq!(raw.len(), 12);

    let mut out = String::with_capacity(14);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && i % 4 == 0 {
            out.push('-');
        }
        out.push(ch);
    }
    out
}

// endregion: generate_approval_code