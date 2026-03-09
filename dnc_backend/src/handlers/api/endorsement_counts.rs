use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::{
    dental_service,
    endorsement,
    endorsement_counts,
};

#[derive(Debug, Serialize)]
pub struct EndorsementCountResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub dental_service_type_id: i32,
    pub sort_index: Option<i32>,
    pub record_tooth: bool,
    pub active: bool,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateEndorsementCountRequest {
    pub dental_service_id: i32,
    pub count: i32,
}

/// GET /api/endorsements/:endorsement_id/counts
#[instrument(skip(state), err(Debug))]
pub async fn get_all_endorsement_counts(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<EndorsementCountResponse>>, StatusCode> {
    tracing::info!("GET /endorsements/{endorsement_id}/counts");

    let endorsement_exists = endorsement::Entity::find_by_id(endorsement_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking endorsement {endorsement_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !endorsement_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    let count_rows = endorsement_counts::Entity::find()
        .filter(endorsement_counts::Column::EndorsementId.eq(endorsement_id))
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed loading endorsement counts for endorsement {endorsement_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let dental_service_ids: Vec<i32> = count_rows.iter().map(|r| r.dental_services_id).collect();

    let dental_services = if dental_service_ids.is_empty() {
        vec![]
    } else {
        dental_service::Entity::find()
            .filter(dental_service::Column::Id.is_in(dental_service_ids))
            .all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed loading dental services for endorsement counts: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    let service_map: std::collections::HashMap<i32, dental_service::Model> =
        dental_services.into_iter().map(|ds| (ds.id, ds)).collect();

    let mut response: Vec<EndorsementCountResponse> = count_rows
        .into_iter()
        .filter_map(|row| {
            let ds = service_map.get(&row.dental_services_id)?;

            Some(EndorsementCountResponse {
                id: row.id,
                endorsement_id: row.endorsement_id,
                dental_service_id: ds.id,
                dental_service_name: ds.name.clone(),
                dental_service_type_id: ds.type_id,
                sort_index: ds.sort_index,
                record_tooth: ds.record_tooth,
                active: ds.active,
                count: row.count,
            })
        })
        .collect();

    response.sort_by(|a, b| {
        a.sort_index
            .unwrap_or(i32::MAX)
            .cmp(&b.sort_index.unwrap_or(i32::MAX))
            .then_with(|| a.dental_service_name.cmp(&b.dental_service_name))
    });

    Ok(Json(response))
}

#[instrument(skip(state, body), err(Debug))]
pub async fn post_endorsement_count(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
    Json(body): Json<CreateEndorsementCountRequest>,
) -> Result<Json<EndorsementCountResponse>, StatusCode> {
    tracing::info!(
        "POST /endorsements/{endorsement_id}/counts dental_service_id={} count={}",
        body.dental_service_id,
        body.count
    );

    let endorsement_exists = endorsement::Entity::find_by_id(endorsement_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking endorsement {endorsement_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !endorsement_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    let dental_service_row = dental_service::Entity::find_by_id(body.dental_service_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking dental service {}: {e:?}", body.dental_service_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::BAD_REQUEST)?;

    let existing = endorsement_counts::Entity::find()
        .filter(endorsement_counts::Column::EndorsementId.eq(endorsement_id))
        .filter(endorsement_counts::Column::DentalServicesId.eq(body.dental_service_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed checking duplicate endorsement_count endorsement_id={} dental_service_id={}: {e:?}",
                endorsement_id,
                body.dental_service_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let am = endorsement_counts::ActiveModel {
        endorsement_id: Set(endorsement_id),
        dental_services_id: Set(body.dental_service_id),
        count: Set(body.count),
        ..Default::default()
    };

    let inserted = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed inserting endorsement_count: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = EndorsementCountResponse {
        id: inserted.id,
        endorsement_id: inserted.endorsement_id,
        dental_service_id: dental_service_row.id,
        dental_service_name: dental_service_row.name,
        dental_service_type_id: dental_service_row.type_id,
        sort_index: dental_service_row.sort_index,
        record_tooth: dental_service_row.record_tooth,
        active: dental_service_row.active,
        count: inserted.count,
    };

    Ok(Json(response))
}