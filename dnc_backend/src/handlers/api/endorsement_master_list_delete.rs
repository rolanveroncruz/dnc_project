use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, TransactionTrait,
};
use serde::Serialize;
use tracing::instrument;

use crate::AppState;
use crate::entities::{master_list, master_list_member};

#[derive(Debug, Serialize)]
pub struct DeleteMasterListsForEndorsementResponse {
    pub endorsement_id: i32,
    pub deleted_master_lists: u64,
    pub deleted_master_list_members: u64,
}

/// DELETE /api/endorsements/:endorsement_id/master_lists
#[instrument(skip(state), err(Debug))]
pub async fn delete_master_lists_for_endorsement_id(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<DeleteMasterListsForEndorsementResponse>, StatusCode> {
    let txn = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::info!("delete_master_lists_for_endorsement_id: {} called", &endorsement_id);

    // 1. Get all master lists with endorsement_id = endorsement_id.
    let master_lists = master_list::Entity::find()
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .all(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. collect into a vector
    let master_list_ids: Vec<i32> = master_lists.iter().map(|ml| ml.id).collect();

    let deleted_master_list_members = if master_list_ids.is_empty() {
        0
    } else {
        master_list_member::Entity::delete_many()
            .filter(master_list_member::Column::MasterListId.is_in(master_list_ids.clone()))
            .exec(&txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .rows_affected
    };

    let deleted_master_lists = if master_list_ids.is_empty() {
        0
    } else {
        master_list::Entity::delete_many()
            .filter(master_list::Column::Id.is_in(master_list_ids))
            .exec(&txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .rows_affected
    };

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DeleteMasterListsForEndorsementResponse {
        endorsement_id,
        deleted_master_lists,
        deleted_master_list_members,
    }))
}