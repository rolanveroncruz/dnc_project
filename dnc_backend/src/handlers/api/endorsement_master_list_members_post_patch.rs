use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set,  EntityTrait,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::master_list_member,
};

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
    pub birth_date: Option<sea_orm::prelude::Date>,
    pub is_active: bool,
    pub last_edited_by: Option<String>,
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
    pub birth_date: Option<Option<sea_orm::prelude::Date>>,
    pub is_active: Option<bool>,
    pub last_edited_by: Option<Option<String>>,
}

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
    pub birth_date: Option<sea_orm::prelude::Date>,
    pub is_active: bool,
    pub last_edited_by: Option<String>,
    pub last_edited_date: sea_orm::prelude::DateTimeWithTimeZone,
}

impl From<master_list_member::Model> for MasterListMemberResponse {
    fn from(model: master_list_member::Model) -> Self {
        Self {
            id: model.id,
            endorsement_id: model.endorsement_id,
            master_list_id: model.master_list_id,
            account_number: model.account_number,
            last_name: model.last_name,
            first_name: model.first_name,
            middle_name: model.middle_name,
            email_address: model.email_address,
            mobile_number: model.mobile_number,
            birth_date: model.birth_date,
            is_active: model.is_active,
            last_edited_by: model.last_edited_by,
            last_edited_date: model.last_edited_date,
        }
    }
}

#[instrument(skip(state, payload), err(Debug))]
pub async fn create_master_list_member(
    State(state): State<AppState>,
    Json(payload): Json<CreateMasterListMemberRequest>,
) -> Result<(StatusCode, Json<MasterListMemberResponse>), (StatusCode, String)> {
    let now = chrono::Utc::now().fixed_offset();

    let new_member = master_list_member::ActiveModel {
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
        last_edited_by: Set(payload.last_edited_by),
        last_edited_date: Set(now),
        ..Default::default()
    };

    let inserted = new_member
        .insert(&state.db)
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted.into())))
}

#[instrument(skip(state, payload), err(Debug))]
pub async fn patch_master_list_member(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchMasterListMemberRequest>,
) -> Result<Json<MasterListMemberResponse>, (StatusCode, String)> {
    let member = master_list_member::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("master_list_member {} not found", id)))?;

    let mut member: master_list_member::ActiveModel = member.into();

    if let Some(value) = payload.endorsement_id {
        member.endorsement_id = Set(value);
    }

    if let Some(value) = payload.master_list_id {
        member.master_list_id = Set(value);
    }

    if let Some(value) = payload.account_number {
        member.account_number = Set(value);
    }

    if let Some(value) = payload.last_name {
        member.last_name = Set(value);
    }

    if let Some(value) = payload.first_name {
        member.first_name = Set(value);
    }

    if let Some(value) = payload.middle_name {
        member.middle_name = Set(value);
    }

    if let Some(value) = payload.email_address {
        member.email_address = Set(value);
    }

    if let Some(value) = payload.mobile_number {
        member.mobile_number = Set(value);
    }

    if let Some(value) = payload.birth_date {
        member.birth_date = Set(value);
    }

    if let Some(value) = payload.is_active {
        member.is_active = Set(value);
    }

    if let Some(value) = payload.last_edited_by {
        member.last_edited_by = Set(value);
    }

    member.last_edited_date = Set(chrono::Utc::now().fixed_offset());

    let updated = member
        .update(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(updated.into()))
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}