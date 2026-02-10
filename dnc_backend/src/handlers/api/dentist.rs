use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, Iterable, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
use sea_orm::prelude::Expr;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;

// Import all the entities you join against
use crate::entities::{
    dentist,
    dentist_contract,
    dentist_history,
    dentist_status,
    tax_classification,
    tax_type,
    account_type,
};

/// Dentist row + lookup "name" fields
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistWithLookups {
    // ---- Dentist columns (match dentist table column names)
    pub id: i32,

    pub prc_no: Option<String>,
    pub prc_expiry_date: Option<sea_orm::prelude::Date>,

    pub last_name: String,
    pub given_name: String,
    pub middle_name: Option<String>,
    pub email: Option<String>,

    pub notes: Option<String>,
    pub retainer_fee: f32,
    pub dentist_status_id: Option<i32>,
    pub dentist_decline_remarks: Option<String>,

    pub dentist_history_id: Option<i32>,
    pub dentist_requested_by: Option<String>,
    pub accre_dentist_contract_id: Option<i32>,
    pub accre_document_code: Option<String>,
    pub accreditation_date: Option<String>,
    pub accre_contract_sent_date: Option<String>,
    pub accre_contract_file_path: Option<String>,
    pub acc_tin: Option<String>,
    pub acc_bank_name: Option<String>,

    pub acc_account_type_id: Option<i32>,
    pub acc_account_name: Option<String>,
    pub acc_account_number: Option<String>,
    pub acc_tax_type_id: Option<i32>,
    pub acc_tax_classification_id: Option<i32>,

    // ---- Lookup names (these must match the column aliases below)
    pub dentist_contract_name: Option<String>,
    pub dentist_history_name: Option<String>,
    pub dentist_status_name: Option<String>,
    pub tax_type_name: Option<String>,
    pub tax_classification_name: Option<String>,
    pub account_type_name: Option<String>,
}

/// Shared query builder so both handlers stay identical.
fn dentist_with_lookups_query() -> sea_orm::Select<dentist::Entity> {
    dentist::Entity::find()
        // left joins so NULL foreign keys don't drop the dentist row
        .join(JoinType::LeftJoin, dentist::Relation::DentistContract.def())
        .join(JoinType::LeftJoin, dentist::Relation::DentistHistory.def())
        .join(JoinType::LeftJoin, dentist::Relation::DentistStatus.def())
        .join(JoinType::LeftJoin, dentist::Relation::TaxType.def())
        .join(JoinType::LeftJoin, dentist::Relation::TaxClassification.def())
        .join(JoinType::LeftJoin, dentist::Relation::AccountType.def())
        // select dentist columns + aliases for lookup names
        .select_only()
        .columns(dentist::Column::iter())
        .expr_as(
            Expr::col((dentist_contract::Entity, dentist_contract::Column::Name)),
            "dentist_contract_name",
        )
        .expr_as(

            Expr::col((dentist_history::Entity, dentist_history::Column::Name)),
            "dentist_history_name",
        )
        .expr_as(
            Expr::col((dentist_status::Entity, dentist_status::Column::Name)),
            "dentist_status_name",
        )
        .expr_as(
            Expr::col((tax_type::Entity, tax_type::Column::Name)),
            "tax_type_name")
        .expr_as(
            Expr::col( (tax_classification::Entity, tax_classification::Column::Name)),
            "tax_classification_name",
        )
        .expr_as(
            Expr::col((account_type::Entity, account_type::Column::Name)),
            "account_type_name",
        )
}

#[instrument(skip(state), err(Debug))]
pub async fn get_all_dentists(
    State(state): State<AppState>,
) -> Result<Json<Vec<DentistWithLookups>>, StatusCode> {
    let rows = dentist_with_lookups_query()
        .order_by_asc(dentist::Column::LastName)
        .order_by_asc(dentist::Column::GivenName)
        .into_model::<DentistWithLookups>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentists: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(rows))
}

#[instrument(skip(state), err(Debug))]
pub async fn get_dentist_from_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<DentistWithLookups>, StatusCode> {
    let row = dentist_with_lookups_query()
        .filter(dentist::Column::Id.eq(id))
        .into_model::<DentistWithLookups>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match row {
        Some(v) => Ok(Json(v)),
        None => Err(StatusCode::NOT_FOUND),
    }
}


#[derive(Debug, Deserialize)]
pub struct CreateDentistRequest {
    // required fields
    pub last_name: String,
    pub given_name: String,
    pub retainer_fee: f32,

    // optional fields
    pub middle_name: Option<String>,
    pub email: Option<String>,

    pub prc_no: Option<String>,
    pub prc_expiry_date: Option<sea_orm::prelude::Date>,
    pub notes: Option<String>,
    pub dentist_decline_remarks: Option<String>,
    pub acc_account_type_id: Option<i32>,

    pub dentist_status_id: Option<i32>,
    pub dentist_history_id: Option<i32>,
    pub dentist_requested_by: Option<String>,

    pub accre_dentist_contract_id: Option<i32>,
    pub accre_document_code: Option<String>,
    pub accreditation_date: Option<String>,
    pub accre_contract_sent_date: Option<String>,
    pub accre_contract_file_path: Option<String>,

    pub acc_tin: Option<String>,
    pub acc_bank_name: Option<String>,
    pub acc_account_name: Option<String>,
    pub acc_account_number: Option<String>,
    pub acc_tax_type_id: Option<i32>,
    pub acc_tax_classification_id: Option<i32>,
}

/// ----- PATCH request (PATCH)
/// For nullable fields, use Option<Option<T>> so you can:
/// - omit the field => no change
/// - set field to null => send `"field": null`
/// - set field value => send `"field": "value"`
#[derive(Debug, Deserialize)]
pub struct PatchDentistRequest {
    pub last_name: Option<String>,
    pub given_name: Option<String>,
    pub retainer_fee: Option<f32>,

    pub middle_name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub prc_no: Option<Option<String>>,
    pub prc_expiry_date: Option<Option<sea_orm::prelude::Date>>,

    pub notes: Option<Option<String>>,
    pub dentist_decline_remarks: Option<Option<String>>,
    pub dentist_status_id: Option<Option<i32>>,
    pub dentist_history_id: Option<Option<i32>>,
    pub dentist_requested_by: Option<Option<String>>,

    pub accre_dentist_contract_id: Option<Option<i32>>,
    pub accre_document_code: Option<Option<String>>,
    pub accreditation_date: Option<Option<String>>,
    pub accre_contract_sent_date: Option<Option<String>>,
    pub accre_contract_file_path: Option<Option<String>>,

    pub acc_tin: Option<Option<String>>,
    pub acc_bank_name: Option<Option<String>>,
    pub acc_account_name: Option<Option<String>>,
    pub acc_account_type_id: Option<Option<i32>>,
    pub acc_account_number: Option<Option<String>>,
    pub acc_tax_type_id: Option<Option<i32>>,
    pub acc_tax_classification_id: Option<Option<i32>>,
}

/// Dentist row + lookup "name" fields

#[instrument(skip(state, body), err(Debug))]
pub async fn create_dentist(
    State(state): State<AppState>,
    Json(body): Json<CreateDentistRequest>,
) -> Result<Json<DentistWithLookups>, StatusCode> {
    // build ActiveModel
    let am = dentist::ActiveModel {
        // id is auto
        last_name: Set(body.last_name),
        given_name: Set(body.given_name),
        middle_name: Set(body.middle_name),
        email: Set(body.email),

        prc_no: Set(body.prc_no),
        prc_expiry_date: Set(body.prc_expiry_date),
        notes: Set(body.notes),

        retainer_fee: Set(body.retainer_fee),

        dentist_status_id: Set(body.dentist_status_id),
        dentist_decline_remarks: Set(body.dentist_decline_remarks),
        acc_account_type_id: Set(body.acc_account_type_id),
        dentist_history_id: Set(body.dentist_history_id),
        dentist_requested_by: Set(body.dentist_requested_by),

        accre_dentist_contract_id: Set(body.accre_dentist_contract_id),
        accre_document_code: Set(body.accre_document_code),
        accreditation_date: Set(body.accreditation_date),
        accre_contract_sent_date: Set(body.accre_contract_sent_date),
        accre_contract_file_path: Set(body.accre_contract_file_path),

        acc_tin: Set(body.acc_tin),
        acc_bank_name: Set(body.acc_bank_name),
        acc_account_name: Set(body.acc_account_name),
        acc_account_number: Set(body.acc_account_number),
        acc_tax_type_id: Set(body.acc_tax_type_id),
        acc_tax_classification_id: Set(body.acc_tax_classification_id),

        ..Default::default()
    };

    let inserted: dentist::Model = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to create dentist: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // return with lookups
    let row = dentist_with_lookups_query()
        .filter(dentist::Column::Id.eq(inserted.id))
        .into_model::<DentistWithLookups>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch created dentist id={}: {e:?}", inserted.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(row))
}

#[instrument(skip(state, body), err(Debug))]
pub async fn patch_dentist(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<PatchDentistRequest>,
) -> Result<Json<DentistWithLookups>, StatusCode> {
    let existing = dentist::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist id={id} for patch: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut am: dentist::ActiveModel = existing.into();

    // non-nullable strings / numbers
    if let Some(v) = body.last_name {
        am.last_name = Set(v);
    }
    if let Some(v) = body.given_name {
        am.given_name = Set(v);
    }
    if let Some(v) = body.retainer_fee {
        am.retainer_fee = Set(v);
    }

    // nullable fields (Option<Option<T>>)
    if let Some(v) = body.middle_name {
        am.middle_name = Set(v);
    }
    if let Some(v) = body.email {
        am.email = Set(v);
    }

    if let Some(v)= body.prc_no {
        am.prc_no = Set(v);
    }
    if let Some(v) = body.prc_expiry_date {
        am.prc_expiry_date = Set(v);
    }
    if let Some(v) = body.notes {
        am.notes = Set(v);
    }
    if let Some(v)=body.dentist_decline_remarks {
        am.dentist_decline_remarks = Set(v);
    }
    if let Some(v) = body.dentist_status_id {
        am.dentist_status_id = Set(v);
    }
    if let Some(v) = body.dentist_history_id {
        am.dentist_history_id = Set(v);
    }
    if let Some(v) = body.dentist_requested_by {
        am.dentist_requested_by = Set(v);
    }

    if let Some(v) = body.accre_dentist_contract_id {
        am.accre_dentist_contract_id = Set(v);
    }
    if let Some(v) = body.accre_document_code {
        am.accre_document_code = Set(v);
    }
    if let Some(v) = body.accreditation_date {
        am.accreditation_date = Set(v);
    }
    if let Some(v) = body.accre_contract_sent_date {
        am.accre_contract_sent_date = Set(v);
    }
    if let Some(v) = body.accre_contract_file_path {
        am.accre_contract_file_path = Set(v);
    }

    if let Some(v) = body.acc_tin {
        am.acc_tin = Set(v);
    }
    if let Some(v) = body.acc_bank_name {
        am.acc_bank_name = Set(v);
    }
    if let Some(v) = body.acc_account_type_id {
        am.acc_account_type_id = Set(v);
    }
    if let Some(v) = body.acc_account_name {
        am.acc_account_name = Set(v);
    }
    if let Some(v) = body.acc_account_number {
        am.acc_account_number = Set(v);
    }
    if let Some(v) = body.acc_tax_type_id {
        am.acc_tax_type_id = Set(v);
    }
    if let Some(v) = body.acc_tax_classification_id {
        am.acc_tax_classification_id = Set(v);
    }

    let updated: dentist::Model = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to patch dentist id={id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let row = dentist_with_lookups_query()
        .filter(dentist::Column::Id.eq(updated.id))
        .into_model::<DentistWithLookups>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch patched dentist id={id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match row {
        Some(v) => Ok(Json(v)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
