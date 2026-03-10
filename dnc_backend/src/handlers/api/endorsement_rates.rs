use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, prelude::Decimal
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::{
    dental_service,
    endorsement,
    endorsement_rates,
};

/// Response row for one endorsement rate
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct EndorsementRateResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub dental_service_type_id: i32,
    pub sort_index: Option<i32>,
    pub record_tooth: bool,
    pub active: bool,
    pub rate: Decimal,
}

/// Request body for POST /endorsements/:endorsement_id/rates
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateEndorsementRateRequest {
    pub dental_service_id: i32,
    pub rate: Decimal,
}

/// GET /api/endorsements/:endorsement_id/rates
#[instrument(skip(state), err(Debug))]
pub async fn get_all_endorsement_rates(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<EndorsementRateResponse>>, StatusCode> {
    tracing::info!("GET /endorsements/{endorsement_id}/rates");

    // Optional but useful: verify the endorsement exists
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

    let rows: Vec<(endorsement_rates::Model, Option<dental_service::Model>)> =
        endorsement_rates::Entity::find()
            .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
            .find_also_related(dental_service::Entity)
            .all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed loading endorsement rates for endorsement {endorsement_id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let mut response: Vec<EndorsementRateResponse> = rows
        .into_iter()
        .filter_map(|(rate_row, dental_service_opt)| {
            let ds = dental_service_opt?;

            Some(EndorsementRateResponse {
                id: rate_row.id,
                endorsement_id: rate_row.endorsement_id,
                dental_service_id: ds.id,
                dental_service_name: ds.name,
                dental_service_type_id: ds.type_id,
                sort_index: ds.sort_index,
                record_tooth: ds.record_tooth,
                active: ds.active,
                rate: rate_row.rate,
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

/// POST /api/endorsements/:endorsement_id/rates
#[instrument(skip(state, body), err(Debug))]
pub async fn post_endorsement_rate(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
    Json(body): Json<CreateEndorsementRateRequest>,
) -> Result<Json<EndorsementRateResponse>, StatusCode> {
    tracing::info!(
        "POST /endorsements/{endorsement_id}/rates dental_service_id={} rate={}",
        body.dental_service_id,
        body.rate
    );

    // 1) Verify endorsement exists
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

    // 2) Verify dental service exists
    let dental_service_row = dental_service::Entity::find_by_id(body.dental_service_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking dental service {}: {e:?}", body.dental_service_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::BAD_REQUEST)?;

    // 3) Optional duplicate protection:
    //    reject if same endorsement + dental service already exists
    let existing = endorsement_rates::Entity::find()
        .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
        .filter(endorsement_rates::Column::DentalServicesId.eq(body.dental_service_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed checking duplicate endorsement_rate endorsement_id={} dental_service_id={}: {e:?}",
                endorsement_id,
                body.dental_service_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 4) Insert
    let am = endorsement_rates::ActiveModel {
        endorsement_id: Set(endorsement_id),
        dental_services_id: Set(body.dental_service_id),
        rate: Set(body.rate),
        ..Default::default()
    };

    let inserted = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed inserting endorsement_rate: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = EndorsementRateResponse {
        id: inserted.id,
        endorsement_id: inserted.endorsement_id,
        dental_service_id: dental_service_row.id,
        dental_service_name: dental_service_row.name,
        dental_service_type_id: dental_service_row.type_id,
        sort_index: dental_service_row.sort_index,
        record_tooth: dental_service_row.record_tooth,
        active: dental_service_row.active,
        rate: inserted.rate,
    };

    Ok(Json(response))
}
/// Full replacement for PUT
#[derive(Debug, Deserialize)]
pub struct UpdateEndorsementRatePutRequest {
    pub dental_service_id: i32,
    pub rate: Decimal,
}

/// Partial update for PATCH
#[derive(Debug, Deserialize)]
pub struct UpdateEndorsementRatePatchRequest {
    pub dental_service_id: Option<i32>,
    pub rate: Option<Decimal>,
}

/// PUT /api/endorsements/:endorsement_id/rates/:rate_id
#[instrument(skip(state, body), err(Debug))]
pub async fn put_endorsement_rate(
    State(state): State<AppState>,
    Path((endorsement_id, rate_id)): Path<(i32, i32)>,
    Json(body): Json<UpdateEndorsementRatePutRequest>,
) -> Result<Json<EndorsementRateResponse>, StatusCode> {
    tracing::info!(
        "PUT /endorsements/{endorsement_id}/rates/{rate_id} dental_service_id={} rate={}",
        body.dental_service_id,
        body.rate
    );

    // 1) Load existing row, scoped to endorsement_id
    let existing = endorsement_rates::Entity::find_by_id(rate_id)
        .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed loading endorsement_rate {rate_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2) Validate dental service exists
    let dental_service_row = dental_service::Entity::find_by_id(body.dental_service_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking dental service {}: {e:?}", body.dental_service_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::BAD_REQUEST)?;

    // 3) Prevent duplicate (same endorsement + same dental service on another row)
    let duplicate = endorsement_rates::Entity::find()
        .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
        .filter(endorsement_rates::Column::DentalServicesId.eq(body.dental_service_id))
        .filter(endorsement_rates::Column::Id.ne(rate_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed checking duplicate endorsement_rate endorsement_id={} dental_service_id={}: {e:?}",
                endorsement_id,
                body.dental_service_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if duplicate.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 4) Update
    let mut am: endorsement_rates::ActiveModel = existing.into();

    am.dental_services_id = Set(body.dental_service_id);
    am.rate = Set(body.rate);

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed updating endorsement_rate {rate_id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = EndorsementRateResponse {
        id: updated.id,
        endorsement_id: updated.endorsement_id,
        dental_service_id: dental_service_row.id,
        dental_service_name: dental_service_row.name,
        dental_service_type_id: dental_service_row.type_id,
        sort_index: dental_service_row.sort_index,
        record_tooth: dental_service_row.record_tooth,
        active: dental_service_row.active,
        rate: updated.rate,
    };

    Ok(Json(response))
}

/// PATCH /api/endorsements/:endorsement_id/rates/:rate_id
#[instrument(skip(state, body), err(Debug))]
pub async fn patch_endorsement_rate(
    State(state): State<AppState>,
    Path((endorsement_id, rate_id)): Path<(i32, i32)>,
    Json(body): Json<UpdateEndorsementRatePatchRequest>,
) -> Result<Json<EndorsementRateResponse>, StatusCode> {
    tracing::info!(
        "PATCH /endorsements/{endorsement_id}/rates/{rate_id} dental_service_id={:?} rate={:?}",
        body.dental_service_id,
        body.rate
    );

    // 1) Load existing row, scoped to endorsement_id
    let existing = endorsement_rates::Entity::find_by_id(rate_id)
        .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed loading endorsement_rate {rate_id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let new_dental_service_id = body.dental_service_id.unwrap_or(existing.dental_services_id);

    // 2) Validate dental service exists
    let dental_service_row = dental_service::Entity::find_by_id(new_dental_service_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed checking dental service {}: {e:?}", new_dental_service_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::BAD_REQUEST)?;

    // 3) Prevent duplicate if dental service changed
    let duplicate = endorsement_rates::Entity::find()
        .filter(endorsement_rates::Column::EndorsementId.eq(endorsement_id))
        .filter(endorsement_rates::Column::DentalServicesId.eq(new_dental_service_id))
        .filter(endorsement_rates::Column::Id.ne(rate_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed checking duplicate endorsement_rate endorsement_id={} dental_service_id={}: {e:?}",
                endorsement_id,
                new_dental_service_id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if duplicate.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // 4) Update only supplied fields
    let mut am: endorsement_rates::ActiveModel = existing.into();

    if let Some(dental_service_id) = body.dental_service_id {
        am.dental_services_id = Set(dental_service_id);
    }

    if let Some(rate) = body.rate {
        am.rate = Set(rate);
    }

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed updating endorsement_rate {rate_id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = EndorsementRateResponse {
        id: updated.id,
        endorsement_id: updated.endorsement_id,
        dental_service_id: dental_service_row.id,
        dental_service_name: dental_service_row.name,
        dental_service_type_id: dental_service_row.type_id,
        sort_index: dental_service_row.sort_index,
        record_tooth: dental_service_row.record_tooth,
        active: dental_service_row.active,
        rate: updated.rate,
    };

    Ok(Json(response))
}
