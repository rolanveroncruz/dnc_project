use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set,
};
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{endorsement, master_list, master_list_member},
};

#[derive(Debug, Serialize, sea_orm::FromQueryResult)]
pub struct MasterListMemberLookupResponse {
    pub master_list_member_id: i32,
    pub endorsement_id: Option<i32>,
    pub endorsement_agreement_corp_number: Option<String>,
    pub master_list_member_account_no: String,
    pub master_list_member_last_name: String,
    pub master_list_member_first_name: String,
    pub master_list_member_middle_name: String,
    pub master_list_member_name: String,
    pub master_list_member_email_address: Option<String>,
    pub master_list_member_mobile_number: Option<String>,
    pub master_list_member_birth_date: Option<Date>,
    pub master_list_member_is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMasterListMemberRequest {
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

#[derive(Debug, Serialize)]
pub struct MasterListMemberMutationResponse {
    pub id: i32,
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
            master_list_member::Column::AccountNumber,
            "master_list_member_account_no",
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
        .expr_as(
            sea_orm::sea_query::Expr::cust(
                r#""master_list_member"."last_name" || ', ' || "master_list_member"."first_name" || ' ' || "master_list_member"."middle_name""#,
            ),
            "master_list_member_name",
        )
        .column_as(
            master_list_member::Column::EmailAddress,
            "master_list_member_email_address",
        )
        .column_as(
            master_list_member::Column::MobileNumber,
            "master_list_member_mobile_number",
        )
        .column_as(
            master_list_member::Column::BirthDate,
            "master_list_member_birth_date",
        )
        .column_as(
            master_list_member::Column::IsActive,
            "master_list_member_is_active",
        )
        .join(
            JoinType::LeftJoin,
            master_list_member::Relation::MasterList.def(),
        )
        .join(
            JoinType::LeftJoin,
            master_list::Relation::Endorsement.def(),
        )
        .order_by_asc(master_list_member::Column::LastName)
        .order_by_asc(master_list_member::Column::FirstName)
        .into_model::<MasterListMemberLookupResponse>()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows))
}

#[instrument(skip(state, payload))]
pub async fn post_master_list_member(
    State(state): State<AppState>,
    Json(payload): Json<CreateMasterListMemberRequest>,
) -> Result<(StatusCode, Json<MasterListMemberMutationResponse>), StatusCode> {
    let active_model = master_list_member::ActiveModel {
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
        Json(MasterListMemberMutationResponse {
            id: inserted.id,
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

#[instrument(skip(state, payload))]
pub async fn patch_master_list_member(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchMasterListMemberRequest>,
) -> Result<Json<MasterListMemberMutationResponse>, StatusCode> {
    let existing = master_list_member::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_model = existing.into_active_model();

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

    Ok(Json(MasterListMemberMutationResponse {
        id: updated.id,
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