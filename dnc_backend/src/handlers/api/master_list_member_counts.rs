
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use serde::Serialize;
use std::collections::HashMap;
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        dental_service,
        endorsement_counts,
        master_list_member,
        verification,
    },
};

#[derive(Debug, Serialize, Clone)]
pub struct EndorsementServiceCountResponse {
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub counts: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberUsedServiceCountResponse {
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub count_used: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberServiceCountSummaryResponse {
    pub dental_service_id: i32,
    pub dental_service_name: String,
    pub counts_allowed: i32,
    pub counts_used: i32,
}

/* =========================================================
   Rule helpers
   ========================================================= */

/// Central place for the "allowed count" rule.
///
/// Rules:
/// - explicit endorsement_counts row => use that count
/// - otherwise if dental_service_type_id == 1 => 999
/// - otherwise => 0
fn resolve_allowed_count(dental_service_type_id: i32, explicit_count: Option<i32>) -> i32 {
    match explicit_count {
        Some(count) => count,
        None if dental_service_type_id == 1 => 999,
        None => 0,
    }
}

/// Central place for deciding whether a verification counts as "used".
///
/// Right now, this counts every verification row as used.
/// Later, if only some status_id values should count, change only this function.
fn verification_status_counts_as_used(_status_id: i32) -> bool {
    true
}

/* =========================================================
   Query helpers
   ========================================================= */

async fn get_member_endorsement_id(
    db: &DatabaseConnection,
    member_id: i32,
) -> Result<i32, DbErr> {
    let member = master_list_member::Entity::find_by_id(member_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("member_id {} not found", member_id)))?;

    Ok(member.endorsement_id)
}

async fn get_all_dental_services(
    db: &DatabaseConnection,
) -> Result<Vec<dental_service::Model>, DbErr> {
    dental_service::Entity::find().all(db).await
}

async fn get_explicit_endorsement_counts(
    db: &DatabaseConnection,
    endorsement_id: i32,
) -> Result<Vec<endorsement_counts::Model>, DbErr> {
    endorsement_counts::Entity::find()
        .filter(endorsement_counts::Column::EndorsementId.eq(endorsement_id))
        .all(db)
        .await
}

async fn get_verifications_for_member(
    db: &DatabaseConnection,
    member_id: i32,
) -> Result<Vec<verification::Model>, DbErr> {
    verification::Entity::find()
        .filter(verification::Column::MemberId.eq(member_id))
        .all(db)
        .await
}

/* =========================================================
   Composition helpers
   ========================================================= */

async fn build_allowed_counts_for_endorsement(
    db: &DatabaseConnection,
    endorsement_id: i32,
) -> Result<Vec<EndorsementServiceCountResponse>, DbErr> {
    let dental_services = get_all_dental_services(db).await?;
    let explicit_counts = get_explicit_endorsement_counts(db, endorsement_id).await?;

    let explicit_count_map: HashMap<i32, i32> = explicit_counts
        .into_iter()
        .map(|row| (row.dental_services_id, row.count))
        .collect();

    let rows = dental_services
        .into_iter()
        .map(|svc| {
            let explicit_count = explicit_count_map.get(&svc.id).copied();
            let counts = resolve_allowed_count(svc.type_id, explicit_count);

            EndorsementServiceCountResponse {
                dental_service_id: svc.id,
                dental_service_name: svc.name,
                counts,
            }
        })
        .collect();

    Ok(rows)
}

async fn build_used_counts_for_member(
    db: &DatabaseConnection,
    member_id: i32,
) -> Result<Vec<MemberUsedServiceCountResponse>, DbErr> {
    let dental_services = get_all_dental_services(db).await?;
    let verifications = get_verifications_for_member(db, member_id).await?;

    let mut used_count_map: HashMap<i32, i32> = HashMap::new();

    for row in verifications {
        if verification_status_counts_as_used(row.status_id) {
            *used_count_map.entry(row.dental_service_id).or_insert(0) += 1;
        }
    }

    let rows = dental_services
        .into_iter()
        .map(|svc| MemberUsedServiceCountResponse {
            dental_service_id: svc.id,
            dental_service_name: svc.name,
            count_used: used_count_map.get(&svc.id).copied().unwrap_or(0),
        })
        .collect();

    Ok(rows)
}

async fn build_count_summary_for_member(
    db: &DatabaseConnection,
    member_id: i32,
) -> Result<Vec<MemberServiceCountSummaryResponse>, DbErr> {
    let endorsement_id = get_member_endorsement_id(db, member_id).await?;

    let allowed_rows = build_allowed_counts_for_endorsement(db, endorsement_id).await?;
    let used_rows = build_used_counts_for_member(db, member_id).await?;

    let used_map: HashMap<i32, i32> = used_rows
        .into_iter()
        .map(|row| (row.dental_service_id, row.count_used))
        .collect();

    let rows = allowed_rows
        .into_iter()
        .map(|allowed| MemberServiceCountSummaryResponse {
            dental_service_id: allowed.dental_service_id,
            dental_service_name: allowed.dental_service_name,
            counts_allowed: allowed.counts,
            counts_used: used_map
                .get(&allowed.dental_service_id)
                .copied()
                .unwrap_or(0),
        })
        .collect();

    Ok(rows)
}

/* =========================================================
   Error helper
   ========================================================= */

fn db_err_to_http(err: DbErr) -> (StatusCode, String) {
    match err {
        DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg),
        other => (StatusCode::INTERNAL_SERVER_ERROR, other.to_string()),
    }
}

/* =========================================================
   Handlers
   ========================================================= */

#[instrument(skip(state), err(Debug))]
pub async fn get_service_counts_for_endorsement_id(
    State(state): State<AppState>,
    Path(endorsement_id): Path<i32>,
) -> Result<Json<Vec<EndorsementServiceCountResponse>>, (StatusCode, String)> {
    let rows = build_allowed_counts_for_endorsement(&state.db, endorsement_id)
        .await
        .map_err(db_err_to_http)?;

    Ok(Json(rows))
}

#[instrument(skip(state), err(Debug))]
pub async fn get_used_service_counts_for_member_id(
    State(state): State<AppState>,
    Path(member_id): Path<i32>,
) -> Result<Json<Vec<MemberUsedServiceCountResponse>>, (StatusCode, String)> {
    let rows = build_used_counts_for_member(&state.db, member_id)
        .await
        .map_err(db_err_to_http)?;

    Ok(Json(rows))
}

#[instrument(skip(state), err(Debug))]
pub async fn get_counts_for_member_id(
    State(state): State<AppState>,
    Path(member_id): Path<i32>,
) -> Result<Json<Vec<MemberServiceCountSummaryResponse>>, (StatusCode, String)> {
    let rows = build_count_summary_for_member(&state.db, member_id)
        .await
        .map_err(db_err_to_http)?;

    Ok(Json(rows))
}