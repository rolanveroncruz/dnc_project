use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::handlers::helpers::role_has_permission_by_data_object_name;
use crate::handlers::structs::AuthUser;
use crate::entities::sea_orm_active_enums::PermissionActionEnum;

use crate::entities::{clinic_capabilities_list, clinic_capability};

/// Returned row for a clinic's assigned capabilities.
/// Includes the junction row and the related capability (if it exists).
#[derive(Debug, Serialize)]
pub struct ClinicCapabilityLinkRow {
    pub id: i32,
    pub clinic_id: i32,
    pub capability_id: i32,
    pub capability: Option<clinic_capability::Model>,
}

#[derive(Debug, Deserialize)]
pub struct AddClinicCapabilityBody {
    pub capability_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct SetClinicCapabilitiesBody {
    pub capability_ids: Vec<i32>,
}

fn to_row(
    link: clinic_capabilities_list::Model,
    cap: Option<clinic_capability::Model>,
) -> ClinicCapabilityLinkRow {
    ClinicCapabilityLinkRow {
        id: link.id,
        clinic_id: link.clinic_id,
        capability_id: link.capability_id,
        capability: cap,
    }
}

/// GET /dental_clinics/:clinic_id/capabilities
#[instrument(skip(state), err(Debug))]
pub async fn get_clinic_capabilities_for_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Path(clinic_id): Path<i32>,
) -> Result<Json<Vec<ClinicCapabilityLinkRow>>, StatusCode> {
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capabilities_list",
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

    let rows = clinic_capabilities_list::Entity::find()
        .filter(clinic_capabilities_list::Column::ClinicId.eq(clinic_id))
        .find_also_related(clinic_capability::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load clinic capabilities list: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(|(link, cap)| to_row(link, cap))
        .collect();

    Ok(Json(rows))
}

/// POST /dental_clinics/:clinic_id/capabilities
/// Body: { "capability_id": 123 }
///
/// Idempotent behavior: if the link already exists, it returns the existing row.
#[instrument(skip(state), err(Debug))]
pub async fn add_clinic_capability_to_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Path(clinic_id): Path<i32>,
    Json(body): Json<AddClinicCapabilityBody>,
) -> Result<Json<ClinicCapabilityLinkRow>, StatusCode> {
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capabilities_list",
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

    // Check existing (so POST is idempotent even without a unique constraint)
    if let Some(existing) = clinic_capabilities_list::Entity::find()
        .filter(clinic_capabilities_list::Column::ClinicId.eq(clinic_id))
        .filter(clinic_capabilities_list::Column::CapabilityId.eq(body.capability_id))
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check existing link: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    {
        let cap = clinic_capability::Entity::find_by_id(existing.capability_id)
            .one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to load related capability: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        return Ok(Json(to_row(existing, cap)));
    }

    let mut am: clinic_capabilities_list::ActiveModel = Default::default();
    am.clinic_id = Set(clinic_id);
    am.capability_id = Set(body.capability_id);

    let insert_res = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to insert clinic capability link: {e:?}");
        // FK/constraint violations are usually "bad request" in this context.
        StatusCode::BAD_REQUEST
    })?;

    let cap = clinic_capability::Entity::find_by_id(insert_res.capability_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load related capability: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(to_row(insert_res, cap)))
}

/// DELETE /dental_clinics/:clinic_id/capabilities/:capability_id
#[instrument(skip(state), err(Debug))]
pub async fn remove_clinic_capability_from_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Path((clinic_id, capability_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capabilities_list",
        // There is no PermissionActionEnum for DELETE, so we use Create instead.
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

    clinic_capabilities_list::Entity::delete_many()
        .filter(clinic_capabilities_list::Column::ClinicId.eq(clinic_id))
        .filter(clinic_capabilities_list::Column::CapabilityId.eq(capability_id))
        .exec(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete clinic capability link: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

/// PUT /dental_clinics/:clinic_id/capabilities
/// Body: { "capability_ids": [1,2,3] }
///
/// Replaces the clinic's capability set in a transaction.
#[instrument(skip(state), err(Debug))]
pub async fn set_clinic_capabilities_for_clinic(
    State(state): State<AppState>,
    user: AuthUser,
    Path(clinic_id): Path<i32>,
    Json(body): Json<SetClinicCapabilitiesBody>,
) -> Result<Json<Vec<ClinicCapabilityLinkRow>>, StatusCode> {
    let has_permission = role_has_permission_by_data_object_name(
        &state.db,
        user.claims.role_id,
        "clinic_capabilities_list",
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

    let txn = state.db.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 1) Remove all existing links for a clinic
    clinic_capabilities_list::Entity::delete_many()
        .filter(clinic_capabilities_list::Column::ClinicId.eq(clinic_id))
        .exec(&txn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to clear existing links: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 2) Insert new links
    // (Dedup to avoid accidental duplicates in the request body)
    let mut ids = body.capability_ids;
    ids.sort_unstable();
    ids.dedup();

    for capability_id in ids {
        let mut am: clinic_capabilities_list::ActiveModel = Default::default();
        am.clinic_id = Set(clinic_id);
        am.capability_id = Set(capability_id);

        am.insert(&txn).await.map_err(|e| {
            tracing::error!("Failed to insert link in bulk set: {e:?}");
            StatusCode::BAD_REQUEST
        })?;
    }

    txn.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Return the updated list
    let rows = clinic_capabilities_list::Entity::find()
        .filter(clinic_capabilities_list::Column::ClinicId.eq(clinic_id))
        .find_also_related(clinic_capability::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load updated links: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(|(link, cap)| to_row(link, cap))
        .collect();

    Ok(Json(rows))
}
