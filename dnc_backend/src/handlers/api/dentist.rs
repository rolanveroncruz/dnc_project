use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, Iterable, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait};
use sea_orm::prelude::Expr;
use serde::Serialize;
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
};

/// Dentist row + lookup "name" fields
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistWithLookups {
    // ---- Dentist columns (match dentist table column names)
    pub id: i32,
    pub last_name: String,
    pub given_name: String,
    pub middle_name: Option<String>,
    pub email: Option<String>,
    pub retainer_fee: f32,
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

    // ---- Lookup names (these must match the column aliases below)
    pub dentist_contract_name: Option<String>,
    pub dentist_history_name: Option<String>,
    pub dentist_status_name: Option<String>,
    pub tax_type_name: Option<String>,
    pub tax_classification_name: Option<String>,
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
