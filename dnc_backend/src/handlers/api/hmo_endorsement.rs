use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::{Date, Decimal},
    ColumnTrait, EntityTrait, QueryFilter, PaginatorTrait,
};
use serde::Serialize;
use tracing::instrument;

use crate::{
    AppState,
    entities::{
        endorsement,
        endorsement_billing_period_type,
        endorsement_company,
        endorsement_type,
        master_list,         // ✅ ADDED
        master_list_member,  // ✅ ADDED
    },
};

#[derive(Debug, Serialize)]
pub struct EndorsementWithLookupsResponse {
    pub id: i32,
    pub hmo_id: i32,
    pub endorsement_company_id: i32,
    pub endorsement_company_name: String,
    pub endorsement_type_id: i32,
    pub endorsement_type_name: String,
    pub agreement_corp_number: Option<String>,
    pub date_start: Date,
    pub date_end: Date,
    pub endorsement_billing_period_type_id: i32,
    pub endorsement_billing_period_type_name: String,
    pub retainer_fee: Option<Decimal>,
    pub remarks: Option<String>,
    pub endorsement_method: Option<String>,
    pub is_active: bool,
    pub master_list_member_count: u64, // ✅ ADDED
}

#[instrument(skip(state))]
async fn count_master_list_members_for_endorsement(
    state: &AppState,
    endorsement_id: i32,
) -> Result<u64, StatusCode> {
    let master_lists = master_list::Entity::find()
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let master_list_ids: Vec<i32> = master_lists.into_iter().map(|ml| ml.id).collect();

    if master_list_ids.is_empty() {
        return Ok(0);
    }

    let count = master_list_member::Entity::find()
        .filter(master_list_member::Column::MasterListId.is_in(master_list_ids))
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(count)
}

#[instrument(skip(state))]
pub async fn get_endorsements_for_hmo_id(
    State(state): State<AppState>,
    Path(hmo_id): Path<i32>,
) -> Result<Json<Vec<EndorsementWithLookupsResponse>>, StatusCode> {
    let endorsements = endorsement::Entity::find()
        .filter(endorsement::Column::HmoId.eq(hmo_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let company_ids: Vec<i32> = endorsements
        .iter()
        .map(|e| e.endorsement_company_id)
        .collect();

    let type_ids: Vec<i32> = endorsements
        .iter()
        .map(|e| e.endorsement_type_id)
        .collect();

    let billing_period_type_ids: Vec<i32> = endorsements
        .iter()
        .map(|e| e.endorsement_billing_period_type_id)
        .collect();

    let companies = endorsement_company::Entity::find()
        .filter(endorsement_company::Column::Id.is_in(company_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let endorsement_types = endorsement_type::Entity::find()
        .filter(endorsement_type::Column::Id.is_in(type_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let billing_period_types = endorsement_billing_period_type::Entity::find()
        .filter(
            endorsement_billing_period_type::Column::Id.is_in(billing_period_type_ids),
        )
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let company_name_by_id: HashMap<i32, String> = companies
        .into_iter()
        .map(|c| (c.id, c.name))
        .collect();

    let endorsement_type_name_by_id: HashMap<i32, String> = endorsement_types
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect();

    let billing_period_type_name_by_id: HashMap<i32, String> = billing_period_types
        .into_iter()
        .map(|b| (b.id, b.name))
        .collect();

    let mut response = Vec::with_capacity(endorsements.len()); // ✅ ADDED

    for endorsement_model in endorsements { // ✅ CHANGED
        let endorsement_company_name = company_name_by_id
            .get(&endorsement_model.endorsement_company_id)
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let endorsement_type_name = endorsement_type_name_by_id
            .get(&endorsement_model.endorsement_type_id)
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let endorsement_billing_period_type_name = billing_period_type_name_by_id
            .get(&endorsement_model.endorsement_billing_period_type_id)
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let master_list_member_count = count_master_list_members_for_endorsement( // ✅ ADDED
                                                                                  &state,
                                                                                  endorsement_model.id,
        )
            .await?;

        response.push(EndorsementWithLookupsResponse {
            id: endorsement_model.id,
            hmo_id: endorsement_model.hmo_id,
            endorsement_company_id: endorsement_model.endorsement_company_id,
            endorsement_company_name,
            endorsement_type_id: endorsement_model.endorsement_type_id,
            endorsement_type_name,
            agreement_corp_number: endorsement_model.agreement_corp_number,
            date_start: endorsement_model.date_start,
            date_end: endorsement_model.date_end,
            endorsement_billing_period_type_id: endorsement_model
                .endorsement_billing_period_type_id,
            endorsement_billing_period_type_name,
            retainer_fee: endorsement_model.retainer_fee,
            remarks: endorsement_model.remarks,
            endorsement_method: endorsement_model.endorsement_method,
            is_active: endorsement_model.is_active,
            master_list_member_count, // ✅ ADDED
        });
    }

    Ok(Json(response))
}