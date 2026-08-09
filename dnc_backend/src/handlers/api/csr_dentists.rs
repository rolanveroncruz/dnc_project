use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

use crate::{
    entities::{
        clinic_capabilities_list,
        clinic_capability,
        dental_clinic,
        dentist,
        dentist_clinic,
    },
    AppState,
};


// ============================================================
// Response structs
// ============================================================

#[derive(Debug, Clone, Serialize)]
pub struct ClinicCapabilityResponse {
    pub id: i32,
    pub name: String,
    pub active: bool,
}


#[derive(Debug, Clone, Serialize)]
pub struct DentistClinicResponse {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub capabilities: Vec<ClinicCapabilityResponse>,
}


#[derive(Debug, Serialize)]
pub struct DentistWithClinicsResponse {
    #[serde(flatten)]
    pub dentist: dentist::Model,

    pub clinics: Vec<DentistClinicResponse>,
}


// ============================================================
// GET ALL DENTISTS
// ============================================================

pub async fn get_all_dentists_for_csr(
    State(state): State<AppState>,
) -> Result<Json<Vec<DentistWithClinicsResponse>>, (StatusCode, String)> {

    // --------------------------------------------------------
    // 1. Load all dentists
    // --------------------------------------------------------

    let dentists = dentist::Entity::find()
        .order_by_asc(dentist::Column::LastName)
        .order_by_asc(dentist::Column::GivenName)
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentists: {err}"),
            )
        })?;


    // --------------------------------------------------------
    // 2. Load dentist -> clinic relationships
    // --------------------------------------------------------

    let dentist_clinic_links = dentist_clinic::Entity::find()
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist-clinic relationships: {err}"),
            )
        })?;


    // Get only clinic IDs that are actually associated with dentists.
    let clinic_ids: Vec<i32> = dentist_clinic_links
        .iter()
        .filter_map(|link| link.clinic_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();


    // --------------------------------------------------------
    // 3. Load those clinics
    // --------------------------------------------------------

    let clinics = if clinic_ids.is_empty() {
        Vec::new()
    } else {
        dental_clinic::Entity::find()
            .filter(dental_clinic::Column::Id.is_in(clinic_ids.clone()))
            .all(&state.db)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load dental clinics: {err}"),
                )
            })?
    };


    // --------------------------------------------------------
    // 4. Load clinic -> capability relationships
    // --------------------------------------------------------

    let capability_links = if clinic_ids.is_empty() {
        Vec::new()
    } else {
        clinic_capabilities_list::Entity::find()
            .filter(
                clinic_capabilities_list::Column::ClinicId
                    .is_in(clinic_ids.clone()),
            )
            .all(&state.db)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load clinic capabilities: {err}"),
                )
            })?
    };


    // Collect capability IDs needed by those clinics.
    let capability_ids: Vec<i32> = capability_links
        .iter()
        .map(|link| link.capability_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();


    // --------------------------------------------------------
    // 5. Load capability definitions
    // --------------------------------------------------------

    let capabilities = if capability_ids.is_empty() {
        Vec::new()
    } else {
        clinic_capability::Entity::find()
            .filter(
                clinic_capability::Column::Id
                    .is_in(capability_ids),
            )
            .all(&state.db)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load capability definitions: {err}"),
                )
            })?
    };


    // ========================================================
    // Assemble data
    // ========================================================


    // --------------------------------------------------------
    // 6. Capability lookup by capability ID
    // --------------------------------------------------------

    let capability_lookup: HashMap<i32, clinic_capability::Model> =
        capabilities
            .into_iter()
            .map(|capability| (capability.id, capability))
            .collect();


    // --------------------------------------------------------
    // 7. Build clinic_id -> Vec<capability>
    // --------------------------------------------------------

    let mut capabilities_by_clinic:
        HashMap<i32, Vec<ClinicCapabilityResponse>> = HashMap::new();

    for link in capability_links {
        if let Some(capability) = capability_lookup.get(&link.capability_id) {
            capabilities_by_clinic
                .entry(link.clinic_id)
                .or_default()
                .push(ClinicCapabilityResponse {
                    id: capability.id,
                    name: capability.name.clone(),
                    active: capability.active,
                });
        }
    }


    // Sort capabilities alphabetically.
    for capabilities in capabilities_by_clinic.values_mut() {
        capabilities.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }


    // --------------------------------------------------------
    // 8. Clinic lookup
    // --------------------------------------------------------

    let clinic_lookup: HashMap<i32, dental_clinic::Model> =
        clinics
            .into_iter()
            .map(|clinic| (clinic.id, clinic))
            .collect();


    // --------------------------------------------------------
    // 9. Build dentist_id -> Vec<clinic>
    // --------------------------------------------------------

    let mut clinics_by_dentist:
        HashMap<i32, Vec<DentistClinicResponse>> = HashMap::new();

    // Because dentist_clinic's unique key includes position_id,
    // theoretically the same dentist/clinic could occur more
    // than once under different positions. Since the response
    // only wants clinics, prevent duplicates here.
    let mut seen_dentist_clinics: HashSet<(i32, i32)> = HashSet::new();

    for link in dentist_clinic_links {
        let Some(clinic_id) = link.clinic_id else {
            continue;
        };

        // Prevent duplicate clinics for the same dentist.
        if !seen_dentist_clinics.insert((link.dentist_id, clinic_id)) {
            continue;
        }

        let Some(clinic) = clinic_lookup.get(&clinic_id) else {
            continue;
        };

        let capabilities = capabilities_by_clinic
            .get(&clinic_id)
            .cloned()
            .unwrap_or_default();

        clinics_by_dentist
            .entry(link.dentist_id)
            .or_default()
            .push(DentistClinicResponse {
                id: clinic.id,
                name: clinic.name.clone(),
                address: clinic.address.clone(),
                capabilities,
            });
    }


    // Sort clinics by clinic name.
    for clinics in clinics_by_dentist.values_mut() {
        clinics.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }


    // --------------------------------------------------------
    // 10. Build final response
    // --------------------------------------------------------

    let response = dentists
        .into_iter()
        .map(|dentist| {
            let clinics = clinics_by_dentist
                .remove(&dentist.id)
                .unwrap_or_default();

            DentistWithClinicsResponse {
                dentist,
                clinics,
            }
        })
        .collect::<Vec<_>>();


    Ok(Json(response))
}