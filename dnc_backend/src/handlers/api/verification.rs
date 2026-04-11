use axum::{extract::State, http::StatusCode, Json};
use axum::extract::Path;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, PaginatorTrait, RelationTrait, Set, Condition};
use sea_orm::sea_query::{Expr,ExprTrait};
use serde::{Serialize, Deserialize};
use tracing::instrument;
use chrono::{ Utc};
use crate::{
    AppState,
    entities::{
        dental_service,
        dentist,
        endorsement,
        master_list_member,
        verification,
        verification_status,
        endorsement_counts,
    },
};
use crate::handlers::AuthUser;
use sea_orm::prelude::{Date};
// region: get all verifications
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
    pub dental_service_is_high_end:bool,
    pub record_tooth: bool,
    pub endorsement_id:i32,
    pub endorsement_agreement_corp_number:Option<String>,
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
    pub dental_service_is_high_end:bool,
    pub record_tooth: bool,

    pub endorsement_id:i32,
    pub endorsement_agreement_corp_number:Option<String>,

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
            master_list_member::Relation::Endorsement.def(),
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
        .expr_as(
            Expr::col(dental_service::Column::TypeId).eq(3),
            "dental_service_is_high_end"
        )
        .column_as(dental_service::Column::RecordTooth, "record_tooth")
        .column_as(master_list_member::Column::EndorsementId, "endorsement_id")
        .column_as(endorsement::Column::AgreementCorpNumber, "endorsement_agreement_corp_number")
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
                dental_service_is_high_end: row.dental_service_is_high_end,
                record_tooth: row.record_tooth,
                endorsement_id: row.endorsement_id,
                endorsement_agreement_corp_number: row.endorsement_agreement_corp_number,
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
// endregion: get all verifications


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
    pub date_service_performed: Option<Date>,
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

fn internal_error(err: DbErr) -> (StatusCode, String) {
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
    pub date_service_performed: Date,
    pub tooth_id: Option<String>,
    pub tooth_service_type_id: Option<i32>,
    pub tooth_surface_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GetApprovalCodeResponse {
    pub reject_code: i32,
    pub reject_message: String,
    pub approval_code: Option<String>,
}

// region: Approval Code Release Check
/*
The following checks need to be performed:
1. Service availment is below endorsement limit.
1. Same dentist, same member, already have 3 approval codes for the same service_performed_day.
2. An approval code was already released for the member but from another dentist in that day.
3. Same service on same tooth id and same surface will be denied.

 */
#[derive(Debug)]
struct ValidationCheckResult {
    code: i32,
    message: &'static str,
}
impl ValidationCheckResult {
    fn ok()->Self{
        Self{
            code: 0,
            message: "ok",
        }
    }
}
async fn check_approval_code_release(
    db: &DatabaseConnection,
    verification_id: i32,
    date_service_performed: Date,
    tooth_id: Option<String>,
    tooth_surface_id: Option<i32>,
    tooth_service_type_id: Option<i32>,
) -> Result<ValidationCheckResult, DbErr> {
    let checks = [
        check_service_allowed_by_endorsement_limit(db,
        verification_id
        ).await?,

        check_other_released_approval_codes_for_same_date(
            db,
            verification_id,
            date_service_performed,
        ).await?,

        check_no_same_dental_service_on_tooth_id_and_surface_and_service_type(
            db,
            verification_id,
            date_service_performed,
            tooth_id,
            tooth_surface_id,
            tooth_service_type_id,
        ) .await?,
    ];

    for check in checks {
        if check.code != 0 {
            return Ok(check);
        }
    }

    Ok(ValidationCheckResult::ok())
}
// check_service_allowed_by_endorsement_limit() checks if the member can still avail of the service
// given the limits set by the endorsement.
async fn check_service_allowed_by_endorsement_limit(
    db: &DatabaseConnection,
    verification_id: i32
)->Result<ValidationCheckResult, DbErr> {

    // ---1 retrieve current_verification from verification_id
    let current_verification = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom(format!("Verification {} not found", verification_id)))?;

    // ---2 retrieve dental_service record from the verification's dental_service_id
    let dental_service = dental_service::Entity::find_by_id(current_verification.dental_service_id)
        .one(db)
        .await?
        .ok_or_else(|| {
            DbErr::Custom(format!(
                "Dental service {} not found",
                current_verification.dental_service_id
            ))
        })?;

    //---3 if unlimited, no need to check further
    if dental_service.is_unlimited == Some(true) {
        return Ok(ValidationCheckResult::ok());
    }

    // ---4 Get endorsement, by first getting the master_list_member record.
    let member = master_list_member::Entity::find_by_id(current_verification.member_id)
        .one(db)
        .await?
        .ok_or_else(|| {
            DbErr::Custom(format!(
                "Master list member {} not found",
                current_verification.member_id
            ))
        })?;

    // ---5 Since dental_service is not unlimited, get the limit from endorsement counts.
    // if record doesn't exist, assume 0.
    let allowed_count = endorsement_counts::Entity::find()
        .filter(endorsement_counts::Column::EndorsementId.eq(member.endorsement_id))
        .filter(
            endorsement_counts::Column::DentalServicesId
                .eq(current_verification.dental_service_id),
        )
        .one(db)
        .await?
        .map(|row| row.count)
        .unwrap_or(0);

    // ---6. Count the total number of already released approval codes for this service for this member.
    let released_count = verification::Entity::find()
        .filter(verification::Column::Id.ne(verification_id))
        .filter(verification::Column::MemberId.eq(current_verification.member_id))
        .filter(
            verification::Column::DentalServiceId.eq(current_verification.dental_service_id),
        )
        .filter(verification::Column::ApprovalCode.is_not_null())
        .filter(verification::Column::StatusId.eq(99))
        .count(db)
        .await? as i32;

    // ----7. Return result
    if released_count >= allowed_count {
        return Ok(ValidationCheckResult {
            code: 6,
            message: "endorsement service limit exceeded for this member",
        });
    }

    Ok(ValidationCheckResult::ok())
}

// check_other_released_approval_codes_for_same_date() checks that only up to 3 approval codes
// can be given to a dentist for a patient in one day.
async fn check_other_released_approval_codes_for_same_date(
    db: &DatabaseConnection,
    verification_id: i32,
    date_service_performed: Date,
) -> Result<ValidationCheckResult, DbErr> {
    let current_verification = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom(format!("Verification {} not found", verification_id)))?;

        let count = verification::Entity::find()
        .filter(verification::Column::Id.ne(verification_id))
        .filter(verification::Column::DentistId.eq(current_verification.dentist_id))
        .filter(verification::Column::MemberId.eq(current_verification.member_id))
        .filter(verification::Column::DateServicePerformed.eq(date_service_performed))
        .filter(verification::Column::ApprovalCode.is_not_null())
        .filter(verification::Column::StatusId.eq(99))
        .count(db)
        .await?;

        if count > 3 {
            return Ok(ValidationCheckResult {
                code: 3,
                message: "approval code release limit exceeded for this dentist, member, and service date",
            });
        }

        Ok(ValidationCheckResult::ok())
}

// check_no_same_dental_service_on_tooth_id_and_surface_and_service_type()
// disallows having the same service on the same tooth and surface if dentists are different.
// But the same dentist is allowed if the service_type are different.
async fn check_no_same_dental_service_on_tooth_id_and_surface_and_service_type(
    db: &DatabaseConnection,
    verification_id: i32,
    date_service_performed: Date,
    tooth_id: Option<String>,
    tooth_surface_id: Option<i32>,
    tooth_service_type_id: Option<i32>,
)->Result<ValidationCheckResult, DbErr> {


    //--- 1. Retrieve the verification record defined by the verification_id.
    let current_verification = verification::Entity::find_by_id(verification_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom(format!("Verification {} not found", verification_id)))?;

    //---2. Create a condition of two verifications different from the one in question,
    // but same member, same service, SAME DATE service performed, and an approval code is present.
    let mut base_condition = Condition::all()
        .add(verification::Column::Id.ne(verification_id))
        .add(verification::Column::MemberId.eq(current_verification.member_id))
        .add(verification::Column::DentalServiceId.eq(current_verification.dental_service_id))
        .add(verification::Column::DateServicePerformed.eq(date_service_performed))
        .add(verification::Column::ApprovalCode.is_not_null())
        .add(verification::Column::StatusId.eq(99));

    // ---2a1 if a tooth_id is present, add the condition of same tooth-id,
    base_condition = match tooth_id {
        Some(ref v) => base_condition.add(verification::Column::ToothId.eq(v.clone())),
        None => base_condition.add(verification::Column::ToothId.is_null()),
    };

    // ---2a2 if a tooth_surface_id is present, add the condition of same tooth_surface_id,
    base_condition = match tooth_surface_id {
        Some(v) => base_condition.add(verification::Column::ToothSurfaceId.eq(v)),
        None => base_condition.add(verification::Column::ToothSurfaceId.is_null()),
    };

    // ---3a Find if another dentist did the other verifications.
    let other_dentist_conflict = verification::Entity::find()
        .filter(base_condition.clone())
        .filter(verification::Column::DentistId.ne(current_verification.dentist_id))
        .one(db)
        .await?;
    // ---3b if that other dentist exists, error.
    if other_dentist_conflict.is_some() {
        return Ok(ValidationCheckResult {
            code: 4,
            message: "same dental service on same day already has an approved verification on this tooth and surface by another dentist",
        });
    }

    // --- 4 Narrow to same dentist.
    let mut same_dentist_same_service_type_condition = base_condition
        .clone()
        .add(verification::Column::DentistId.eq(current_verification.dentist_id));

    // ---4a Compare service types
    same_dentist_same_service_type_condition = match tooth_service_type_id {
        Some(v) => same_dentist_same_service_type_condition
            .add(verification::Column::ToothServiceTypeId.eq(v)),
        None => same_dentist_same_service_type_condition
            .add(verification::Column::ToothServiceTypeId.is_null()),
    };
    // --- 4b find a conflict of same service type.
    let same_dentist_same_service_type_conflict = verification::Entity::find()
        .filter(same_dentist_same_service_type_condition)
        .one(db)
        .await?;

    // --- 4c If found, return error
    if same_dentist_same_service_type_conflict.is_some() {
        return Ok(ValidationCheckResult {
            code: 5,
            message: "same dental service already has an approved verification on this tooth, surface, and service type for this dentist",
        });
    }

    // ----5 all good
    Ok(ValidationCheckResult::ok())
}

// region: Get Approval Code For Verification ID
#[instrument(skip(state, auth_user, payload), err(Debug))]
pub async fn get_approval_code_for_verification_id(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(verification_id): Path<i32>,
    Json(payload): Json<GetApprovalCodeRequest>,
) -> Result<Json<GetApprovalCodeResponse>, (StatusCode, String)> {

    // --- 1. Retrieve the verification row defined by verification_id
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

    // --- 2. Do checks if the approval code could be released.
    let validation = check_approval_code_release(
        &state.db,
        verification_id,
        payload.date_service_performed,
        payload.tooth_id.clone(),
        payload.tooth_surface_id,
        payload.tooth_service_type_id,
    )
        .await
        .map_err(internal_error)?;

    // --- 2a. If there was an issue, return the code and message.
    if validation.code != 0 {
        return Ok(Json(GetApprovalCodeResponse {
            reject_code: validation.code,
            reject_message: validation.message.to_string(),
            approval_code: None,
        }));
    }
    // --- 3. There were no problems. All clear. Save the approval code.
    let approval_code = generate_approval_code(verification_id);
    let mut verification_active: verification::ActiveModel = verification_model.into();
    verification_active.date_service_performed = Set(Some(payload.date_service_performed));
    verification_active.tooth_id = Set(payload.tooth_id.clone());
    verification_active.tooth_surface_id = Set(payload.tooth_surface_id);
    verification_active.tooth_service_type_id = Set(payload.tooth_service_type_id);
    verification_active.approved_by = Set(Some(auth_user.claims.email.clone()));
    verification_active.approval_date = Set(Some(Utc::now().into()));
    verification_active.approval_code = Set(Some(approval_code.clone()));
    verification_active.status_id = Set(99); // 99 is "Done"

    verification_active
        .update(&state.db)
        .await
        .map_err(internal_error)?;

    // --- 4. Return the response.
    Ok(Json(GetApprovalCodeResponse {
        reject_code: 0,
        reject_message: "ok".to_string(),
        approval_code: Some(approval_code),
    }))
}






// endregion: Get Approval Code for Verification ID


// region: generate_approval_code
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

 type HmacSha256 = Hmac<Sha256>;

 const APPROVAL_CODE_ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const ID_PART_LEN: usize = 5;
const TAG_PART_LEN: usize = 4;

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
    let mask_25 = u64::from_be_bytes(mask_arr) & ((1u64 << 25) - 1);

    let obfuscated_id = id_u64 ^ mask_25;

    let mut tag_mac =
        <HmacSha256 as KeyInit>::new_from_slice(secret.as_bytes())
            .expect("invalid HMAC key");
    tag_mac.update(b"approval-code-tag:v1:");
    tag_mac.update(verification_id.to_string().as_bytes());
    let tag_bytes = tag_mac.finalize().into_bytes();

    let mut tag_arr = [0u8; 4];
    tag_arr.copy_from_slice(&tag_bytes[..4]);
    let tag_20 = (u32::from_be_bytes(tag_arr) >> 12) & ((1u32 << 20) - 1);

    let id_part = encode_base32_fixed(obfuscated_id, ID_PART_LEN);
    let tag_part = encode_base32_fixed(tag_20 as u64, TAG_PART_LEN);

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
    debug_assert_eq!(raw.len(), 9);

    let mut out = String::with_capacity(11);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push('-');
        }
        out.push(ch);
    }
    out
}


// endregion: generate_approval_code

// endregion: Get Approval Code
