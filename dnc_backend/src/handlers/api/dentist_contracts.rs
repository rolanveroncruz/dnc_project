use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
};
use serde::Serialize;
use tracing::instrument;

use crate::AppState;
use crate::handlers::structs::AuthUser;

// CHANGE THIS: import your permission helper + enum if you want these endpoints permission-gated
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

// Entities
use crate::entities::{
    dentist_contract,
    dentist_contract_service_rates,
    dental_service,
};

#[derive(Debug, Serialize)]
pub struct DentistContractRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub last_modified_by: String,
    pub last_modified_on: sea_orm::prelude::DateTimeWithTimeZone,
}

#[derive(Debug, Serialize)]
pub struct DentistContractServiceRateRow {
    pub id: i32,
    pub dentist_contract_id: Option<i32>,
    pub service_id: Option<i32>,
    pub service_name: Option<String>, // derived from related dental_service
    pub rate: f32,
}

#[derive(Debug, Serialize)]
pub struct DentistContractWithRates {
    pub contract: DentistContractRow,
    pub rates: Vec<DentistContractServiceRateRow>,
}
//--------------------------
// GET /api/get_all_dentist_contracts()
//
//--------------------------
#[instrument(skip(state), err(Debug))]
pub async fn get_all_dentist_contracts(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<DentistContractRow>>, StatusCode> {
    // 1) Permission check (optional but recommended; matches your existing style)
    // CHANGE THIS: set the correct data_object_name string used in your permission table.
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dentist_contract",                 // CHANGE THIS if your data_object name differs
        PermissionActionEnum::Read,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    // 2) Query all contracts
    let models = dentist_contract::Entity::find()
        .order_by_asc(dentist_contract::Column::Id)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist contracts: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 3) Map to DTOs
    let rows = models
        .into_iter()
        .map(|m| DentistContractRow {
            id: m.id,
            name: m.name,
            description: m.description,
            active: m.active,
            last_modified_by: m.last_modified_by,
            last_modified_on: m.last_modified_on,
        })
        .collect();

    Ok(Json(rows))
}


//--------------------------
// GET /api/get_dentist_contract/{:id}
//
//--------------------------
#[instrument(skip(state), err(Debug))]
pub async fn get_dentist_contract(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<DentistContractWithRates>, StatusCode> {
    // 1) Permission check
    // CHANGE THIS: set correct data_object name if needed
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dentist_contract",                 // CHANGE THIS if needed
        PermissionActionEnum::Read,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    // 2) Fetch the contract
    let contract = dentist_contract::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist contract {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let contract_row = DentistContractRow {
        id: contract.id,
        name: contract.name,
        description: contract.description,
        active: contract.active,
        last_modified_by: contract.last_modified_by,
        last_modified_on: contract.last_modified_on,
    };

    // 3) Fetch service rates + join dental_service to get service_name
    //
    // We do a left join via `find_also_related`, so even if service_id is NULL
    // or the related dental_service row is missing, you still get the rate row.
    let rate_pairs: Vec<(dentist_contract_service_rates::Model, Option<dental_service::Model>)> =
        dentist_contract_service_rates::Entity::find()
            .filter(dentist_contract_service_rates::Column::DentistContractId.eq(id))
            .find_also_related(dental_service::Entity)
            .order_by_asc(dentist_contract_service_rates::Column::Id)
            .all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch service rates for contract {id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let rates = rate_pairs
        .into_iter()
        .map(|(rate_model, svc_opt)| DentistContractServiceRateRow {
            id: rate_model.id,
            dentist_contract_id: rate_model.dentist_contract_id,
            service_id: rate_model.service_id,
            service_name: svc_opt.map(|s| s.name),
            rate: rate_model.rate,
        })
        .collect();

    Ok(Json(DentistContractWithRates {
        contract: contract_row,
        rates,
    }))
}
