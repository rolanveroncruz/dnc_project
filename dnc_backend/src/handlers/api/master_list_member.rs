use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ ColumnTrait, EntityTrait, JoinType,
              QueryFilter, QuerySelect, RelationTrait, };
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        master_list_member,
        endorsement,
        endorsement_company,
    },

};

#[derive(Debug, Serialize)]
pub struct MasterListMemberResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMasterListMemberRequest {
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatchMasterListMemberRequest {
    pub endorsement_id: Option<i32>,
    pub master_list_id: Option<Option<i32>>,
    pub account_number: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub email_address: Option<Option<String>>,
    pub mobile_number: Option<Option<String>>,
    pub birth_date: Option<Option<Date>>,
    pub is_active: Option<bool>,
}




#[derive(Debug, Serialize, sea_orm::FromQueryResult)]
pub struct MasterListMemberForEndorsementResponse {
    pub endorsement_company_name: String,
    pub agreement_corp_number: Option<String>,

    pub id: i32,
    pub endorsement_id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: Option<String>,
    pub mobile_number: Option<String>,
    pub birth_date: Option<Date>,
    pub is_active: bool,
}
#[instrument(skip(state), err(Debug))]
pub async fn get_master_list_members_for_endorsement(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<MasterListMemberForEndorsementResponse>>, (StatusCode, String)> {
    let rows = master_list_member::Entity::find()
        .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::Endorsement.def(),
        )
        .join(
            JoinType::InnerJoin,
            endorsement::Relation::EndorsementCompany.def(),
        )
        .select_only()
        .column_as(
            endorsement_company::Column::Name,
            "endorsement_company_name",
        )
        .column_as(
            endorsement::Column::AgreementCorpNumber,
            "agreement_corp_number",
        )
        .column(master_list_member::Column::Id)
        .column(master_list_member::Column::EndorsementId)
        .column(master_list_member::Column::MasterListId)
        .column(master_list_member::Column::AccountNumber)
        .column(master_list_member::Column::LastName)
        .column(master_list_member::Column::FirstName)
        .column(master_list_member::Column::MiddleName)
        .column(master_list_member::Column::EmailAddress)
        .column(master_list_member::Column::MobileNumber)
        .column(master_list_member::Column::BirthDate)
        .column(master_list_member::Column::IsActive)
        .into_model::<MasterListMemberForEndorsementResponse>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(rows))
}

fn internal_error(err: sea_orm::DbErr) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}