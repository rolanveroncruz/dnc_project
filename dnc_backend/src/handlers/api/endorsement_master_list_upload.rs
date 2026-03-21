use std::collections::HashSet;
use std::io::Cursor;

use axum::{
    extract::{Extension, Multipart, Path, State},
    http::StatusCode,
    Json,
};
use calamine::{Data, Reader, Xlsx};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, Condition, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::Serialize;
use tracing::instrument;

use crate::{
    AppState,
    entities::{endorsement, master_list, master_list_member},
    handlers::structs::AuthUser,
};

/*
UploadMasterListMemberRow is a row from the spreadsheet
*/
#[derive(Debug, Serialize)]
pub struct UploadedMasterListMemberRow {
    pub row_number: usize,
    pub corporate_number: String,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
}

/*
ExistingDuplicateRow is a row from the master_list_member table
that is a duplicate.
*/
#[derive(Debug, Serialize)]
pub struct ExistingDuplicateRow {
    pub id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct DuplicateRowResponse {
    pub uploaded_row: UploadedMasterListMemberRow,
    pub existing_row: ExistingDuplicateRow,
}

#[derive(Debug, Serialize)]
pub struct InsertedMasterListMemberRow {
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
}

#[derive(Debug, Serialize)]
pub struct UploadEndorsementMasterListResponse {
    pub master_list_id: i32,
    pub file_name: String,
    pub endorsement_id: i32,
    pub inserted_count: usize,
    pub inserted_rows: Vec<InsertedMasterListMemberRow>,
    pub skipped_corporate_number_mismatch_count: usize,
    pub duplicate_count: usize,
    pub duplicates: Vec<DuplicateRowResponse>,
}

/*
Collection of XL row information that will be saved
*/
#[derive(Debug, Clone)]
struct PendingMasterListMemberRow {
    account_number: String,
    last_name: String,
    first_name: String,
    middle_name: String,
}

/// POST /api/endorsements/:endorsement_id/master-list/upload
///
/// Expects multipart/form-data with a file field named "file".
#[instrument(skip(state, multipart), err(Debug))]
pub async fn upload_endorsement_master_list(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
    Extension(auth_user): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadEndorsementMasterListResponse>), StatusCode> {
    tracing::info!("called upload_endorsement_master_list");

    // 1) Load endorsement
    let endorsement_row = endorsement::Entity::find_by_id(endorsement_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 1a) Extract agreement_corp_number
    let agreement_corp_number = endorsement_row
        .agreement_corp_number
        .clone()
        .ok_or(StatusCode::BAD_REQUEST)?
        .trim()
        .to_string();

    // 2) Read uploaded file from multipart
    let mut uploaded_file_name: Option<String> = None;
    let mut uploaded_file_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let field_name = field.name().unwrap_or_default().to_string();

        if field_name == "file" {
            uploaded_file_name = field.file_name().map(|s| s.to_string());
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            uploaded_file_bytes = Some(data.to_vec());
            break;
        }
    }

    let file_name = uploaded_file_name.unwrap_or_else(|| "uploaded_master_list.xlsx".to_string());
    let file_bytes = uploaded_file_bytes.ok_or(StatusCode::BAD_REQUEST)?;

    // 3) Parse XLSX
    let cursor = Cursor::new(file_bytes);
    let mut workbook: Xlsx<_> = Xlsx::new(cursor).map_err(|_| StatusCode::BAD_REQUEST)?;

    let range = workbook
        .worksheet_range_at(0)
        .ok_or(StatusCode::BAD_REQUEST)?
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 4) Start transaction
    let txn = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uploaded_by = Some(auth_user.claims.email.clone());

    let mut duplicates: Vec<DuplicateRowResponse> = Vec::new();
    let mut pending_rows: Vec<PendingMasterListMemberRow> = Vec::new();
    let mut pending_account_numbers: HashSet<String> = HashSet::new();
    let mut inserted_rows: Vec<InsertedMasterListMemberRow> = Vec::new();
    let mut inserted_count = 0usize;
    let mut skipped_corporate_number_mismatch_count = 0usize;

    // Excel row 2 means skip header row (index 0)
    for (zero_based_idx, row) in range.rows().enumerate().skip(1) {
        let row_number = zero_based_idx + 1;

        // These are your already-adjusted zero-based indexes:
        // 5 = CorporateNumber
        // 6 = AccountNumber
        // 7 = LastName
        // 8 = FirstName
        // 9 = MiddleName
        let corporate_number = get_cell_string(row, 5);
        let account_number = get_cell_string(row, 6);
        let last_name = get_cell_string(row, 7);
        let first_name = get_cell_string(row, 8);
        let middle_name = get_cell_string(row, 9);

        tracing::info!(
            "row_number:{}: corporate_number:{}\t a_n:{}\t l_name:{}\t f_name:{}\t m_name:{}",
            row_number,
            corporate_number,
            account_number,
            last_name,
            first_name,
            middle_name
        );

        if account_number.is_empty() || corporate_number.is_empty() {
            continue;
        }

        if corporate_number != agreement_corp_number {
            skipped_corporate_number_mismatch_count += 1;
            continue;
        }

        let uploaded_row = UploadedMasterListMemberRow {
            row_number,
            corporate_number,
            account_number: account_number.clone(),
            last_name: last_name.clone(),
            first_name: first_name.clone(),
            middle_name: middle_name.clone(),
        };

        // Ignore duplicate account numbers within the same uploaded file completely
        if pending_account_numbers.contains(&account_number) {
            continue;
        }

        // Exact same account + exact same names already in DB => ignore
        let exact_existing = match master_list_member::Entity::find()
            .filter(master_list_member::Column::AccountNumber.eq(account_number.clone()))
            .filter(master_list_member::Column::LastName.eq(last_name.clone()))
            .filter(master_list_member::Column::FirstName.eq(first_name.clone()))
            .filter(master_list_member::Column::MiddleName.eq(middle_name.clone()))
            .one(&txn)
            .await
        {
            Ok(existing) => existing,
            Err(_) => {
                let _ = txn.rollback().await;
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if exact_existing.is_some() {
            continue;
        }

        // Same account + different names already in DB => duplicate
        let existing = match master_list_member::Entity::find()
            .filter(master_list_member::Column::AccountNumber.eq(account_number.clone()))
            .filter(
                Condition::any()
                    .add(master_list_member::Column::LastName.ne(last_name.clone()))
                    .add(master_list_member::Column::FirstName.ne(first_name.clone()))
                    .add(master_list_member::Column::MiddleName.ne(middle_name.clone())),
            )
            .one(&txn)
            .await
        {
            Ok(existing) => existing,
            Err(_) => {
                let _ = txn.rollback().await;
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if let Some(existing) = existing {
            duplicates.push(DuplicateRowResponse {
                uploaded_row,
                existing_row: ExistingDuplicateRow {
                    id: existing.id,
                    master_list_id: existing.master_list_id,
                    account_number: existing.account_number,
                    last_name: existing.last_name,
                    first_name: existing.first_name,
                    middle_name: existing.middle_name,
                    email_address: existing.email_address,
                    mobile_number: existing.mobile_number,
                    birth_date: existing.birth_date.map(|d| d.to_string()),
                    is_active: existing.is_active,
                },
            });
            continue;
        }

        pending_account_numbers.insert(account_number.clone());
        pending_rows.push(PendingMasterListMemberRow {
            account_number,
            last_name,
            first_name,
            middle_name,
        });
    }

    // Only create master_list if at least one row will actually be inserted
    if pending_rows.is_empty() {
        let _ = txn.rollback().await;
        return Err(StatusCode::BAD_REQUEST);
    }

    let new_master_list = master_list::ActiveModel {
        file_name: Set(file_name.clone()),
        endorsement_id: Set(Some(endorsement_id)),
        uploaded_by: Set(uploaded_by),
        upload_date: Set(Some(Utc::now().fixed_offset())),
        ..Default::default()
    };

    let master_list_row = match new_master_list.insert(&txn).await {
        Ok(row) => row,
        Err(_) => {
            let _ = txn.rollback().await;
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    for pending in pending_rows {
        let inserted_row_for_response = InsertedMasterListMemberRow {
            account_number: pending.account_number.clone(),
            last_name: pending.last_name.clone(),
            first_name: pending.first_name.clone(),
            middle_name: pending.middle_name.clone(),
        };

        let insert_result = master_list_member::ActiveModel {
            master_list_id: Set(Some(master_list_row.id)),
            account_number: Set(pending.account_number),
            last_name: Set(pending.last_name),
            first_name: Set(pending.first_name),
            middle_name: Set(pending.middle_name),
            email_address: Set(None),
            mobile_number: Set(None),
            birth_date: Set(None),
            is_active: Set(true),
            ..Default::default()
        }
            .insert(&txn)
            .await;

        match insert_result {
            Ok(_) => {
                inserted_count += 1;
                inserted_rows.push(inserted_row_for_response);
            }
            Err(_) => {
                let _ = txn.rollback().await;
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(UploadEndorsementMasterListResponse {
            master_list_id: master_list_row.id,
            file_name,
            endorsement_id,
            inserted_count,
            inserted_rows,
            skipped_corporate_number_mismatch_count,
            duplicate_count: duplicates.len(),
            duplicates,
        }),
    ))
}

fn get_cell_string(row: &[Data], idx: usize) -> String {
    match row.get(idx) {
        Some(Data::String(s)) => s.trim().to_string(),
        Some(Data::Float(f)) => {
            if f.fract() == 0.0 {
                (*f as i64).to_string()
            } else {
                f.to_string()
            }
        }
        Some(Data::Int(i)) => i.to_string(),
        Some(Data::Bool(b)) => b.to_string(),
        Some(Data::DateTime(dt)) => dt.to_string(),
        Some(Data::DateTimeIso(s)) => s.trim().to_string(),
        Some(Data::DurationIso(s)) => s.trim().to_string(),
        Some(Data::Error(_)) => String::new(),
        Some(Data::Empty) => String::new(),
        None => String::new(),
    }
}