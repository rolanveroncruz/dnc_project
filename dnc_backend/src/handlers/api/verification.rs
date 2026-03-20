use std::collections::{HashMap, HashSet};

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        dental_service,
        dentist,
        master_list_member,
        verification,
        verification_status,
    },
};

#[derive(Debug, Serialize)]
pub struct VerificationWithLookupsResponse {
    pub verification_id: i32,
    pub date: sea_orm::prelude::DateTimeWithTimeZone,
    pub dentist_name: String,
    pub master_list_member_name: String,
    pub dental_service_name: String,
    pub status: String,
}

#[instrument(skip(state))]
pub async fn get_all_verifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<VerificationWithLookupsResponse>>, StatusCode> {
    let verifications = verification::Entity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if verifications.is_empty() {
        return Ok(Json(vec![]));
    }

    let dentist_ids: Vec<i32> = verifications
        .iter()
        .map(|v| v.dentist_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let member_ids: Vec<i32> = verifications
        .iter()
        .map(|v| v.member_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let status_ids: Vec<i32> = verifications
        .iter()
        .map(|v| v.status_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let dental_service_ids: Vec<i32> = verifications
        .iter()
        .map(|v| v.dental_service_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let dentists = dentist::Entity::find()
        .filter(dentist::Column::Id.is_in(dentist_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let members = master_list_member::Entity::find()
        .filter(master_list_member::Column::Id.is_in(member_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let statuses = verification_status::Entity::find()
        .filter(verification_status::Column::Id.is_in(status_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dental_services = dental_service::Entity::find()
        .filter(dental_service::Column::Id.is_in(dental_service_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dentist_map: HashMap<i32, dentist::Model> =
        dentists.into_iter().map(|d| (d.id, d)).collect();

    let member_map: HashMap<i32, master_list_member::Model> =
        members.into_iter().map(|m| (m.id, m)).collect();

    let status_map: HashMap<i32, verification_status::Model> =
        statuses.into_iter().map(|s| (s.id, s)).collect();

    let dental_service_map: HashMap<i32, dental_service::Model> =
        dental_services.into_iter().map(|ds| (ds.id, ds)).collect();

    let response = verifications
        .into_iter()
        .map(|v| {
            let dentist = dentist_map
                .get(&v.dentist_id)
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let member = member_map
                .get(&v.member_id)
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let status = status_map
                .get(&v.status_id)
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let dental_service = dental_service_map
                .get(&v.dental_service_id)
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let dentist_name = match &dentist.middle_name {
                Some(middle) if !middle.trim().is_empty() => {
                    format!("{}, {} {}", dentist.last_name, dentist.given_name, middle)
                }
                _ => format!("{}, {}", dentist.last_name, dentist.given_name),
            };

            let member_middle = member.middle_name.trim();
            let master_list_member_name = if member_middle.is_empty() {
                format!("{}, {}", member.last_name, member.first_name)
            } else {
                format!("{}, {} {}", member.last_name, member.first_name, member_middle)
            };

            Ok(VerificationWithLookupsResponse {
                verification_id: v.id,
                date: v.date,
                dentist_name,
                master_list_member_name,
                dental_service_name: dental_service.name.clone(),
                status: status.name.clone(),
            })
        })
        .collect::<Result<Vec<_>, StatusCode>>()?;

    Ok(Json(response))
}