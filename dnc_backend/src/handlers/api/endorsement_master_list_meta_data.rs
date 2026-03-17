use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, PaginatorTrait, RelationTrait,
};
use serde::Serialize;
use tracing::instrument;

use crate::AppState;
use crate::entities::{master_list, master_list_member};

#[derive(Debug, Serialize)]
pub struct MasterListMetaDataResponse {
    pub id: i32,
    pub file_name: String,
    pub uploaded_by: Option<String>,
    pub upload_date: Option<sea_orm::prelude::DateTimeWithTimeZone>,
    pub total_rows: u64,
}

/// GET /api/endorsements/:endorsement_id/master_list_metadata
#[instrument(skip(state), err(Debug))]
pub async fn get_master_list_meta_data_for_endorsement_id(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<MasterListMetaDataResponse>, StatusCode> {
    // Get the latest uploaded master_list for this endorsement_id.
    // "Latest" is determined by upload_date descending, then id descending as tie-breaker.
    let latest_master_list = master_list::Entity::find()
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .order_by_desc(master_list::Column::UploadDate)
        .order_by_desc(master_list::Column::Id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NO_CONTENT)?;

    // Count all master_list_member rows whose master_list belongs to this endorsement_id.
    let total_rows = master_list_member::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            master_list_member::Relation::MasterList.def(),
        )
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MasterListMetaDataResponse {
        id: latest_master_list.id,
        file_name: latest_master_list.file_name,
        uploaded_by: latest_master_list.uploaded_by,
        upload_date: latest_master_list.upload_date,
        total_rows,
    }))
}