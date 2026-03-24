use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, IntoActiveModel,
              QueryFilter, QuerySelect, RelationTrait, Set};
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

#[instrument(skip(state))]
pub async fn post_master_list_member(
    State(state): State<AppState>,
    Json(payload): Json<CreateMasterListMemberRequest>,
) -> Result<(StatusCode, Json<MasterListMemberResponse>), StatusCode> {
    let active_model = master_list_member::ActiveModel {
        endorsement_id: Set(payload.endorsement_id),
        master_list_id: Set(payload.master_list_id),
        account_number: Set(payload.account_number),
        last_name: Set(payload.last_name),
        first_name: Set(payload.first_name),
        middle_name: Set(payload.middle_name),
        email_address: Set(payload.email_address),
        mobile_number: Set(payload.mobile_number),
        birth_date: Set(payload.birth_date),
        is_active: Set(payload.is_active),
        ..Default::default()
    };

    let inserted = active_model
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(MasterListMemberResponse {
            id: inserted.id,
            endorsement_id: inserted.endorsement_id,
            master_list_id: inserted.master_list_id,
            account_number: inserted.account_number,
            last_name: inserted.last_name,
            first_name: inserted.first_name,
            middle_name: inserted.middle_name,
            email_address: inserted.email_address,
            mobile_number: inserted.mobile_number,
            birth_date: inserted.birth_date,
            is_active: inserted.is_active,
        }),
    ))
}

#[instrument(skip(state))]
pub async fn patch_master_list_member(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchMasterListMemberRequest>,
) -> Result<Json<MasterListMemberResponse>, StatusCode> {
    let member = master_list_member::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_model = member.into_active_model();

    if let Some(endorsement_id) = payload.endorsement_id {
        active_model.endorsement_id = Set(endorsement_id);
    }
    if let Some(master_list_id) = payload.master_list_id {
        active_model.master_list_id = Set(master_list_id);
    }
    if let Some(account_number) = payload.account_number {
        active_model.account_number = Set(account_number);
    }
    if let Some(last_name) = payload.last_name {
        active_model.last_name = Set(last_name);
    }
    if let Some(first_name) = payload.first_name {
        active_model.first_name = Set(first_name);
    }
    if let Some(middle_name) = payload.middle_name {
        active_model.middle_name = Set(middle_name);
    }
    if let Some(email_address) = payload.email_address {
        active_model.email_address = Set(email_address);
    }
    if let Some(mobile_number) = payload.mobile_number {
        active_model.mobile_number = Set(mobile_number);
    }
    if let Some(birth_date) = payload.birth_date {
        active_model.birth_date = Set(birth_date);
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = Set(is_active);
    }

    let updated = active_model
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MasterListMemberResponse {
        id: updated.id,
        endorsement_id: updated.endorsement_id,
        master_list_id: updated.master_list_id,
        account_number: updated.account_number,
        last_name: updated.last_name,
        first_name: updated.first_name,
        middle_name: updated.middle_name,
        email_address: updated.email_address,
        mobile_number: updated.mobile_number,
        birth_date: updated.birth_date,
        is_active: updated.is_active,
    }))
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