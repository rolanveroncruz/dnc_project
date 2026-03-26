use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    AppState,
    entities::endorsement_billing_rule,
};

#[derive(Debug, Serialize)]
pub struct EndorsementBillingRuleResponse {
    pub id: i32,
    pub endorsement_id: i32,
    pub min_count: i32,
    pub max_count: i32,
    pub rate: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct CreateEndorsementBillingRuleRequest {
    pub endorsement_id: i32,
    pub min_count: i32,
    pub max_count: i32,
    pub rate: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct PatchEndorsementBillingRuleRequest {
    pub min_count: Option<i32>,
    pub max_count: Option<i32>,
    pub rate: Option<Decimal>,
}

#[instrument(skip(state))]
pub async fn get_billing_rules_for_endorsement_id(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<EndorsementBillingRuleResponse>>, (StatusCode, String)> {
    let rules = endorsement_billing_rule::Entity::find()
        .filter(endorsement_billing_rule::Column::EndorsementId.eq(endorsement_id))
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    let response = rules
        .into_iter()
        .map(|rule| EndorsementBillingRuleResponse {
            id: rule.id,
            endorsement_id: rule.endorsement_id,
            min_count: rule.min_count,
            max_count: rule.max_count,
            rate: rule.rate,
        })
        .collect();

    Ok(Json(response))
}

#[instrument(skip(state, payload))]
pub async fn post_billing_rule(
    State(state): State<AppState>,
    Json(payload): Json<CreateEndorsementBillingRuleRequest>,
) -> Result<(StatusCode, Json<EndorsementBillingRuleResponse>), (StatusCode, String)> {
    if payload.min_count > payload.max_count {
        return Err((
            StatusCode::BAD_REQUEST,
            "min_count cannot be greater than max_count".to_string(),
        ));
    }

    let new_rule = endorsement_billing_rule::ActiveModel {
        endorsement_id: Set(payload.endorsement_id),
        min_count: Set(payload.min_count),
        max_count: Set(payload.max_count),
        rate: Set(payload.rate),
        ..Default::default()
    };

    let inserted = new_rule.insert(&state.db).await.map_err(internal_error)?;

    Ok((
        StatusCode::CREATED,
        Json(EndorsementBillingRuleResponse {
            id: inserted.id,
            endorsement_id: inserted.endorsement_id,
            min_count: inserted.min_count,
            max_count: inserted.max_count,
            rate: inserted.rate,
        }),
    ))
}

#[instrument(skip(state, payload))]
pub async fn patch_billing_rule(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchEndorsementBillingRuleRequest>,
) -> Result<Json<EndorsementBillingRuleResponse>, (StatusCode, String)> {
    let rule = endorsement_billing_rule::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("endorsement billing rule with id {} not found", id),
            )
        })?;

    let new_min_count = payload.min_count.unwrap_or(rule.min_count);
    let new_max_count = payload.max_count.unwrap_or(rule.max_count);

    if new_min_count > new_max_count {
        return Err((
            StatusCode::BAD_REQUEST,
            "min_count cannot be greater than max_count".to_string(),
        ));
    }

    let mut active_model = rule.into_active_model();

    if let Some(min_count) = payload.min_count {
        active_model.min_count = Set(min_count);
    }

    if let Some(max_count) = payload.max_count {
        active_model.max_count = Set(max_count);
    }

    if let Some(rate) = payload.rate {
        active_model.rate = Set(rate);
    }

    let updated = active_model.update(&state.db).await.map_err(internal_error)?;

    Ok(Json(EndorsementBillingRuleResponse {
        id: updated.id,
        endorsement_id: updated.endorsement_id,
        min_count: updated.min_count,
        max_count: updated.max_count,
        rate: updated.rate,
    }))
}

#[instrument(skip(state))]
pub async fn delete_billing_rule(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rule = endorsement_billing_rule::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("endorsement billing rule with id {} not found", id),
            )
        })?;

    let active_model = rule.into_active_model();
    active_model.delete(&state.db).await.map_err(internal_error)?;

    Ok(StatusCode::NO_CONTENT)
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}