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
        dentist_company_relations,
        dentist_hmo_relations,
        endorsement_company,
        hmo,
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

#[derive(Debug, Clone, Serialize)]
pub struct DentistCompanyResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistHmoResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct DentistWithClinicsResponse {
    #[serde(flatten)]
    pub dentist: dentist::Model,

    pub clinics: Vec<DentistClinicResponse>,

    pub exclusive_to_companies: Vec<DentistCompanyResponse>,
    pub except_for_companies: Vec<DentistCompanyResponse>,

    pub exclusive_to_hmos: Vec<DentistHmoResponse>,
    pub except_for_hmos: Vec<DentistHmoResponse>,
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

    let clinic_ids: Vec<i32> = dentist_clinic_links
        .iter()
        .filter_map(|link| link.clinic_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // --------------------------------------------------------
    // 3. Load clinics
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
    // Company relationships
    // ========================================================

    // --------------------------------------------------------
    // 6. Load dentist -> company relationships
    // --------------------------------------------------------

    let company_relation_links = dentist_company_relations::Entity::find()
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist-company relationships: {err}"),
            )
        })?;

    let company_ids: Vec<i32> = company_relation_links
        .iter()
        .map(|link| link.company_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // --------------------------------------------------------
    // 7. Load company definitions
    // --------------------------------------------------------

    let companies = if company_ids.is_empty() {
        Vec::new()
    } else {
        endorsement_company::Entity::find()
            .filter(
                endorsement_company::Column::Id
                    .is_in(company_ids),
            )
            .all(&state.db)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load companies: {err}"),
                )
            })?
    };

    // ========================================================
    // HMO relationships
    // ========================================================

    // --------------------------------------------------------
    // 8. Load dentist -> HMO relationships
    // --------------------------------------------------------

    let hmo_relation_links = dentist_hmo_relations::Entity::find()
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load dentist-HMO relationships: {err}"),
            )
        })?;

    let hmo_ids: Vec<i32> = hmo_relation_links
        .iter()
        .map(|link| link.hmo_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // --------------------------------------------------------
    // 9. Load HMO definitions
    // --------------------------------------------------------

    let hmos = if hmo_ids.is_empty() {
        Vec::new()
    } else {
        hmo::Entity::find()
            .filter(hmo::Column::Id.is_in(hmo_ids))
            .all(&state.db)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load HMOs: {err}"),
                )
            })?
    };

    // ========================================================
    // Assemble clinic/capability data
    // ========================================================

    // --------------------------------------------------------
    // 10. Capability lookup by capability ID
    // --------------------------------------------------------

    let capability_lookup: HashMap<i32, clinic_capability::Model> =
        capabilities
            .into_iter()
            .map(|capability| (capability.id, capability))
            .collect();

    // --------------------------------------------------------
    // 11. clinic_id -> capabilities
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

    for capabilities in capabilities_by_clinic.values_mut() {
        capabilities.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    // --------------------------------------------------------
    // 12. Clinic lookup
    // --------------------------------------------------------

    let clinic_lookup: HashMap<i32, dental_clinic::Model> =
        clinics
            .into_iter()
            .map(|clinic| (clinic.id, clinic))
            .collect();

    // --------------------------------------------------------
    // 13. dentist_id -> clinics
    // --------------------------------------------------------

    let mut clinics_by_dentist:
        HashMap<i32, Vec<DentistClinicResponse>> = HashMap::new();

    let mut seen_dentist_clinics: HashSet<(i32, i32)> = HashSet::new();

    for link in dentist_clinic_links {
        let Some(clinic_id) = link.clinic_id else {
            continue;
        };

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

    for clinics in clinics_by_dentist.values_mut() {
        clinics.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    // ========================================================
    // Assemble company relationship data
    // ========================================================

    // --------------------------------------------------------
    // 14. Company lookup
    // --------------------------------------------------------

    let company_lookup: HashMap<i32, endorsement_company::Model> =
        companies
            .into_iter()
            .map(|company| (company.id, company))
            .collect();

    let mut exclusive_companies_by_dentist:
        HashMap<i32, Vec<DentistCompanyResponse>> = HashMap::new();

    let mut except_companies_by_dentist:
        HashMap<i32, Vec<DentistCompanyResponse>> = HashMap::new();

    // Avoid accidental duplicate relationship rows.
    let mut seen_company_relations: HashSet<(i32, i32, bool)> =
        HashSet::new();

    for link in company_relation_links {
        let Some(is_exclusive) = link.is_exclusive_to_company else {
            continue;
        };

        if !seen_company_relations.insert((
            link.dentist_id,
            link.company_id,
            is_exclusive,
        )) {
            continue;
        }

        let Some(company) = company_lookup.get(&link.company_id) else {
            continue;
        };

        let response = DentistCompanyResponse {
            id: company.id,
            name: company.name.clone(),
        };

        if is_exclusive {
            exclusive_companies_by_dentist
                .entry(link.dentist_id)
                .or_default()
                .push(response);
        } else {
            except_companies_by_dentist
                .entry(link.dentist_id)
                .or_default()
                .push(response);
        }
    }

    // Sort company names alphabetically.
    for companies in exclusive_companies_by_dentist.values_mut() {
        companies.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    for companies in except_companies_by_dentist.values_mut() {
        companies.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    // ========================================================
    // Assemble HMO relationship data
    // ========================================================

    // --------------------------------------------------------
    // 15. HMO lookup
    // --------------------------------------------------------

    let hmo_lookup: HashMap<i32, hmo::Model> =
        hmos
            .into_iter()
            .map(|hmo| (hmo.id, hmo))
            .collect();

    let mut exclusive_hmos_by_dentist:
        HashMap<i32, Vec<DentistHmoResponse>> = HashMap::new();

    let mut except_hmos_by_dentist:
        HashMap<i32, Vec<DentistHmoResponse>> = HashMap::new();

    let mut seen_hmo_relations: HashSet<(i32, i32, bool)> =
        HashSet::new();

    for link in hmo_relation_links {
        let Some(is_exclusive) = link.is_exclusive_to_hmo else {
            continue;
        };

        if !seen_hmo_relations.insert((
            link.dentist_id,
            link.hmo_id,
            is_exclusive,
        )) {
            continue;
        }

        let Some(hmo) = hmo_lookup.get(&link.hmo_id) else {
            continue;
        };

        let response = DentistHmoResponse {
            id: hmo.id,
            name: hmo.short_name.clone(),
        };

        if is_exclusive {
            exclusive_hmos_by_dentist
                .entry(link.dentist_id)
                .or_default()
                .push(response);
        } else {
            except_hmos_by_dentist
                .entry(link.dentist_id)
                .or_default()
                .push(response);
        }
    }

    // Sort HMO short names alphabetically.
    for hmos in exclusive_hmos_by_dentist.values_mut() {
        hmos.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    for hmos in except_hmos_by_dentist.values_mut() {
        hmos.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
        });
    }

    // ========================================================
    // 16. Build final response
    // ========================================================

    let response = dentists
        .into_iter()
        .map(|dentist| {
            let dentist_id = dentist.id;

            let clinics = clinics_by_dentist
                .remove(&dentist_id)
                .unwrap_or_default();

            let exclusive_to_companies = exclusive_companies_by_dentist
                .remove(&dentist_id)
                .unwrap_or_default();

            let except_for_companies = except_companies_by_dentist
                .remove(&dentist_id)
                .unwrap_or_default();

            let exclusive_to_hmos = exclusive_hmos_by_dentist
                .remove(&dentist_id)
                .unwrap_or_default();

            let except_for_hmos = except_hmos_by_dentist
                .remove(&dentist_id)
                .unwrap_or_default();

            DentistWithClinicsResponse {
                dentist,
                clinics,
                exclusive_to_companies,
                except_for_companies,
                exclusive_to_hmos,
                except_for_hmos,
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(response))
}