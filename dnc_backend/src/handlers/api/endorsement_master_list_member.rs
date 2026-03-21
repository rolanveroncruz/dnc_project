use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::prelude::Date;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::{master_list, master_list_member},
};

#[derive(Debug, Serialize)]
pub struct EndorsementMasterListMemberResponse {
    pub file_name: String,
    pub master_list_member_id: i32,
    pub account_number: String,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct MasterListMemberResponse {
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

#[derive(Debug, Deserialize)]
pub struct SetMasterListMemberActiveRequest {
    pub is_active: bool,
}

/// GET /api/endorsements/:endorsement_id/master_lists
#[instrument(skip(state), err(Debug))]
pub async fn get_master_list_for_endorsement(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<EndorsementMasterListMemberResponse>>, StatusCode> {
    let master_lists = master_list::Entity::find()
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if master_lists.is_empty() {
        return Ok(Json(vec![]));
    }

    let master_list_ids: Vec<i32> = master_lists.iter().map(|ml| ml.id).collect();

    let file_name_by_master_list_id: HashMap<i32, String> = master_lists
        .into_iter()
        .map(|ml| (ml.id, ml.file_name))
        .collect();

    let members = master_list_member::Entity::find()
        .filter(master_list_member::Column::MasterListId.is_in(master_list_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = members
        .into_iter()
        .filter_map(|m| {
            m.master_list_id.and_then(|master_list_id| {
                file_name_by_master_list_id
                    .get(&master_list_id)
                    .map(|file_name| EndorsementMasterListMemberResponse {
                        file_name: file_name.clone(),
                        master_list_member_id: m.id,
                        account_number: m.account_number,
                        last_name: m.last_name,
                        first_name: m.first_name,
                        middle_name: m.middle_name,
                        is_active: m.is_active,
                    })
            })
        })
        .collect();

    Ok(Json(response))
}

/// PATCH /api/master_list_members/:master_list_member_id/active
#[instrument(skip(state), err(Debug))]
pub async fn set_master_list_member_active(
    State(state): State<AppState>,
    Path(master_list_member_id): Path<i32>,
    Json(payload): Json<SetMasterListMemberActiveRequest>,
) -> Result<Json<MasterListMemberResponse>, StatusCode> {
    let member = master_list_member::Entity::find_by_id(master_list_member_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut member_active_model: master_list_member::ActiveModel = member.into();
    member_active_model.is_active = Set(payload.is_active);

    let updated = member_active_model
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MasterListMemberResponse {
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