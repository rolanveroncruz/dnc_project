use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait,
              ActiveValue::NotSet, QueryFilter, QueryOrder, Set, TransactionTrait};
use serde::{Serialize, Deserialize};
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
    pub dentist_contract_id: i32,
    pub service_id: i32,
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
// returns a list of {id, name, description, active }
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
// returns a single contract with rates:
// {
//   contract: {id, name, description, active },
//   rates: [{id, dentist_contract_id, service_id, service_name, rate }]
// }
//
//--------------------------
#[instrument(skip(state), err(Debug))]
pub async fn get_dentist_contract(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<DentistContractWithRates>, StatusCode> {
    // 1) Permission check
    // CHANGE THIS: set the correct data_object name if needed
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


/*
 * POST and PATCh DTOs.
 */
#[derive(Debug, Deserialize)]
pub struct CreateDentistContractRequest {
    pub name: String,
    pub description: String,
    pub active: bool,

    // Optional: create initial rates in the same request
    pub rates: Option<Vec<UpsertDentistContractRateRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDentistContractRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,

    // Optional: if present, we REPLACE ALL rates
    // If absent, we leave rates unchanged.
    pub rates: Option<Vec<UpsertDentistContractRateRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertDentistContractRateRequest {
    pub service_id: i32,
    pub rate: f32,
}

/*
 * POST and PATCH rate handlers.
 */

async fn replace_all_rates(
    tx: &DatabaseTransaction,
    dentist_contract_id: i32,
    rates: Vec<UpsertDentistContractRateRequest>,
) -> Result<(), DbErr> {
    use sea_orm::DbErr;

    // 1) Delete existing rates for this contract
    dentist_contract_service_rates::Entity::delete_many()
        .filter(dentist_contract_service_rates::Column::DentistContractId.eq(dentist_contract_id))
        .exec(tx)
        .await?;

    // 2) Insert new rates
    for r in rates {
        // Optional: validate (avoid negatives)
        if r.rate.is_nan() || r.rate < 0.0 {
            return Err(DbErr::Custom(format!(
                "Invalid rate for service_id {}: {}",
                r.service_id, r.rate
            )));
        }

        let am = dentist_contract_service_rates::ActiveModel {
            id: NotSet, // ignored by auto-increment; safe with SeaORM
            dentist_contract_id: Set(dentist_contract_id),
            service_id: Set(r.service_id),
            rate: Set(r.rate),
        };

        am.insert(tx).await?;
    }

    Ok(())
}
/*
 * POST /api/post_dentist_contract/
 * data: {name, description, active, rates: [{service_id, rate}]}
 *
*/


#[instrument(skip(state), err(Debug))]
pub async fn post_dentist_contract(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateDentistContractRequest>,
) -> Result<Json<DentistContractWithRates>, StatusCode> {
    // Permission: Create
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dentist_contract", // CHANGE THIS if your data_object name differs
        PermissionActionEnum::Create,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    // Basic validation (optional)
    if payload.name.trim().is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let tx = state.db.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 1) Insert contract
    let contract_am = dentist_contract::ActiveModel {
        id: NotSet, // ignored if auto-increment
        name: Set(payload.name),
        description: Set(payload.description),
        active: Set(payload.active),

        // CHANGE THIS: set last_modified_by to whatever your AuthUser stores (email/username/etc)
        last_modified_by: Set(user.claims.email.clone()),

        // CHANGE THIS: if you already have a "now()" helper, use it.
        last_modified_on: NotSet,
    };

    let contract_model = contract_am.insert(&tx).await.map_err(|e| {
        tracing::error!("Failed to insert dentist_contract: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 2) Optional rates
    if let Some(rates) = payload.rates {
        // Replace-all on create means "insert these"
        replace_all_rates(&tx, contract_model.id, rates)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert contract rates: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 3) Return the created object using your existing getter logic:
    //    (reuse your read query to include service_name)
    get_dentist_contract(State(state), user, Path(contract_model.id)).await
}

/*
 * PATCH /api/patch_dentist_contract/{:id}
 * data: {name, description, active, rates: [{service_id, rate}]}
 */


#[instrument(skip(state), err(Debug))]
pub async fn patch_dentist_contract(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchDentistContractRequest>,
) -> Result<Json<DentistContractWithRates>, StatusCode> {
    // Permission: Update
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dentist_contract", // CHANGE THIS if needed
        PermissionActionEnum::Update,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    let tx = state.db.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 1) Load existing
    let existing = dentist_contract::Entity::find_by_id(id)
        .one(&tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist_contract {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2) Patch fields
    let mut am: dentist_contract::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        if name.trim().is_empty() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        am.name = Set(name);
    }
    if let Some(desc) = payload.description {
        am.description = Set(desc);
    }
    if let Some(active) = payload.active {
        am.active = Set(active);
    }

    // Always update audit fields on patch
    // CHANGE THIS: pick the correct user field
    am.last_modified_by = Set(user.claims.email.clone());
    am.last_modified_on = Set(chrono::Utc::now().into());

    am.update(&tx).await.map_err(|e| {
        tracing::error!("Failed to update dentist_contract {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 3) Optional: replace rates
    if let Some(rates) = payload.rates {
        replace_all_rates(&tx, id, rates).await.map_err(|e| {
            tracing::error!("Failed to replace rates for contract {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 4) Return the updated object (contract + rates + service_name)
    get_dentist_contract(State(state), user, Path(id)).await
}

#[derive(Debug, Deserialize)]
pub struct PatchDentistContractRatesRequest {
    pub rates: Vec<UpsertDentistContractRateRequest>,
}

/*
 * PATCH /api/patch_dentist_contract_rates/{:id}
 * data: {rates: [{service_id, rate}]}
 * 
 */
#[instrument(skip(state), err(Debug))]
pub async fn patch_dentist_contract_rates(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<PatchDentistContractRatesRequest>,
) -> Result<Json<DentistContractWithRates>, StatusCode> {
    // Permission: Update
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "dentist_contract", // CHANGE THIS if needed
        PermissionActionEnum::Update,
    )
        .await
        .map_err(|e| {
            tracing::error!("Failed to check permission: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if !has_permission {
        return Err(StatusCode::FORBIDDEN);
    }

    let tx = state.db.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Ensure the contract exists (otherwise you’d be inserting orphan rows)
    let exists = dentist_contract::Entity::find_by_id(id)
        .one(&tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dentist_contract {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    replace_all_rates(&tx, id, payload.rates).await.map_err(|e| {
        tracing::error!("Failed to replace rates for contract {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Optional: also update audit fields on the contract when rates change
    // (recommended so UI shows last_modified_on changes when rates are edited)
    let mut am: dentist_contract::ActiveModel =
        dentist_contract::Entity::find_by_id(id)
            .one(&tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to re-fetch dentist_contract {id}: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)?
            .into();

    // CHANGE THIS: pick the correct user field
    am.last_modified_by = Set(user.claims.email.clone());
    am.last_modified_on = Set(chrono::Utc::now().into());

    am.update(&tx).await.map_err(|e| {
        tracing::error!("Failed to update audit fields for contract {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_dentist_contract(State(state), user, Path(id)).await
}

