use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryOrder,
    QuerySelect, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{endorsement, master_list, master_list_member},
};

#[derive(Debug, Serialize)]
pub struct MasterListMemberLookupResponse {
    pub master_list_member_id: i32,
    pub endorsement_id: Option<i32>,
    pub endorsement_agreement_corp_number: Option<String>,
    pub master_list_member_last_name: String,
    pub master_list_member_first_name: String,
    pub master_list_member_middle_name: String,
    pub master_list_member_name: String,
    pub master_list_member_is_active: bool,
}

#[derive(Debug, sea_orm::FromQueryResult)]
struct MasterListMemberLookupRow {
    pub master_list_member_id: i32,
    pub endorsement_id: Option<i32>,
    pub endorsement_agreement_corp_number: Option<String>,
    pub master_list_member_last_name: String,
    pub master_list_member_first_name: String,
    pub master_list_member_middle_name: String,
    pub master_list_member_is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMasterListMemberRequest {
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: String,
    pub mobile_number: Option<String>,
    pub birth_date: Option<sea_orm::prelude::Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct PutMasterListMemberRequest {
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: String,
    pub mobile_number: Option<String>,
    pub birth_date: Option<sea_orm::prelude::Date>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatchMasterListMemberRequest {
    pub master_list_id: Option<Option<i32>>,
    pub account_number: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub email_address: Option<String>,
    pub mobile_number: Option<Option<String>>,
    pub birth_date: Option<Option<sea_orm::prelude::Date>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MasterListMemberResponse {
    pub id: i32,
    pub master_list_id: Option<i32>,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub email_address: String,
    pub mobile_number: Option<String>,
    pub birth_date: Option<sea_orm::prelude::Date>,
    pub is_active: bool,
}

impl From<master_list_member::Model> for MasterListMemberResponse {
    fn from(value: master_list_member::Model) -> Self {
        Self {
            id: value.id,
            master_list_id: value.master_list_id,
            account_number: value.account_number,
            last_name: value.last_name,
            first_name: value.first_name,
            middle_name: value.middle_name,
            email_address: value.email_address,
            mobile_number: value.mobile_number,
            birth_date: value.birth_date,
            is_active: value.is_active,
        }
    }
}

fn format_full_name(last_name: &str, first_name: &str, middle_name: &str) -> String {
    let middle = middle_name.trim();

    if middle.is_empty() {
        format!("{}, {}", last_name.trim(), first_name.trim())
    } else {
        format!("{}, {} {}", last_name.trim(), first_name.trim(), middle)
    }
}

#[instrument(skip(state))]
pub async fn get_all_master_list_members(
    State(state): State<AppState>,
) -> Result<Json<Vec<MasterListMemberLookupResponse>>, StatusCode> {
    let rows = master_list_member::Entity::find()
        .select_only()
        .column_as(master_list_member::Column::Id, "master_list_member_id")
        .column_as(master_list::Column::EndorsementId, "endorsement_id")
        .column_as(
            endorsement::Column::AgreementCorpNumber,
            "endorsement_agreement_corp_number",
        )
        .column_as(
            master_list_member::Column::LastName,
            "master_list_member_last_name",
        )
        .column_as(
            master_list_member::Column::FirstName,
            "master_list_member_first_name",
        )
        .column_as(
            master_list_member::Column::MiddleName,
            "master_list_member_middle_name",
        )
        .column_as(
            master_list_member::Column::IsActive,
            "master_list_member_is_active",
        )
        .join(
            JoinType::LeftJoin,
            master_list_member::Relation::MasterList.def(),
        )
        .join(JoinType::LeftJoin, master_list::Relation::Endorsement.def())
        .order_by_asc(master_list_member::Column::LastName)
        .order_by_asc(master_list_member::Column::FirstName)
        .order_by_asc(master_list_member::Column::MiddleName)
        .into_model::<MasterListMemberLookupRow>()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = rows
        .into_iter()
        .map(|row| MasterListMemberLookupResponse {
            master_list_member_id: row.master_list_member_id,
            endorsement_id: row.endorsement_id,
            endorsement_agreement_corp_number: row.endorsement_agreement_corp_number,
            master_list_member_last_name: row.master_list_member_last_name.clone(),
            master_list_member_first_name: row.master_list_member_first_name.clone(),
            master_list_member_middle_name: row.master_list_member_middle_name.clone(),
            master_list_member_name: format_full_name(
                &row.master_list_member_last_name,
                &row.master_list_member_first_name,
                &row.master_list_member_middle_name,
            ),
            master_list_member_is_active: row.master_list_member_is_active,
        })
        .collect();

    Ok(Json(response))
}

#[instrument(skip(state, payload))]
pub async fn create_master_list_member(
    State(state): State<AppState>,
    Json(payload): Json<CreateMasterListMemberRequest>,
) -> Result<(StatusCode, Json<MasterListMemberResponse>), StatusCode> {
    let active_model = master_list_member::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        master_list_id: Set(payload.master_list_id),
        account_number: Set(payload.account_number),
        last_name: Set(payload.last_name),
        first_name: Set(payload.first_name),
        middle_name: Set(payload.middle_name),
        email_address: Set(payload.email_address),
        mobile_number: Set(payload.mobile_number),
        birth_date: Set(payload.birth_date),
        is_active: Set(payload.is_active),
    };

    let inserted = active_model
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(inserted.into())))
}

#[instrument(skip(state, payload))]
pub async fn put_master_list_member(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PutMasterListMemberRequest>,
) -> Result<Json<MasterListMemberResponse>, StatusCode> {
    let existing = master_list_member::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_model = existing.into_active_model();

    active_model.master_list_id = Set(payload.master_list_id);
    active_model.account_number = Set(payload.account_number);
    active_model.last_name = Set(payload.last_name);
    active_model.first_name = Set(payload.first_name);
    active_model.middle_name = Set(payload.middle_name);
    active_model.email_address = Set(payload.email_address);
    active_model.mobile_number = Set(payload.mobile_number);
    active_model.birth_date = Set(payload.birth_date);
    active_model.is_active = Set(payload.is_active);

    let updated = active_model
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated.into()))
}

#[instrument(skip(state, payload))]
pub async fn patch_master_list_member(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchMasterListMemberRequest>,
) -> Result<Json<MasterListMemberResponse>, StatusCode> {
    let existing = master_list_member::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_model = existing.into_active_model();

    if let Some(value) = payload.master_list_id {
        active_model.master_list_id = Set(value);
    }
    if let Some(value) = payload.account_number {
        active_model.account_number = Set(value);
    }
    if let Some(value) = payload.last_name {
        active_model.last_name = Set(value);
    }
    if let Some(value) = payload.first_name {
        active_model.first_name = Set(value);
    }
    if let Some(value) = payload.middle_name {
        active_model.middle_name = Set(value);
    }
    if let Some(value) = payload.email_address {
        active_model.email_address = Set(value);
    }
    if let Some(value) = payload.mobile_number {
        active_model.mobile_number = Set(value);
    }
    if let Some(value) = payload.birth_date {
        active_model.birth_date = Set(value);
    }
    if let Some(value) = payload.is_active {
        active_model.is_active = Set(value);
    }

    let updated = active_model
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated.into()))
}