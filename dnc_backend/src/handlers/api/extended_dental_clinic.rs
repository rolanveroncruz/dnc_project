use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tracing::instrument;

use crate::AppState;
use crate::entities::{dental_clinic, clinic_capability, clinic_capabilities_list,
                      city, province, region};

#[derive(Debug, Serialize)]
pub struct ClinicWithCapabilities {
    // clinic fields (copy whatever you want to expose)
    pub id: i32,
    pub name: String,
    pub owner_name: Option<String>,
    pub address: String,
    pub city_id: Option<i32>,
    pub city_name: Option<String>,
    pub province_id: Option<i32>,
    pub province_name: Option<String>,
    pub region_id: Option<i32>,
    pub region_name: Option<String>,
    pub zip_code: Option<String>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub email: Option<String>,
    pub schedule: Option<String>,
    pub active: Option<bool>,
    pub last_modified_by: String,
    pub last_modified_on: sea_orm::prelude::DateTimeWithTimeZone,

    // dynamic: "capability_name" -> boolean
    pub capabilities: HashMap<String, bool>,
}

/// GET /api/clinics-with-capabilities
#[instrument(skip(state), err(Debug))]
pub async fn get_all_clinics_and_capabilities(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClinicWithCapabilities>>, StatusCode> {

    // 1) Get the universe of capability names (usually only active ones)
    let models: Vec<clinic_capability::Model> = clinic_capability::Entity::find()
        .filter(clinic_capability::Column::Active.eq(true))
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load capability names: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?; // <-- IMPORTANT: this ? makes `models` a Vec<Model>, not a Select

    let all_capability_names: Vec<String> = models
        .into_iter()
        .map(|m| m.name)
        .collect();


    // 2) Load all clinics
    let clinics: Vec<dental_clinic::Model> = dental_clinic::Entity::find()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load clinics: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    // 2.5) ✅ ADDED: Load clinic -> city/province/region names in one query
    // Result rows are: (clinic_id, city_id, city_name, province_id, province_name, region_id, region_name)
    let clinic_locations: Vec<(i32, Option<i32>, Option<String>, Option<i32>, Option<String>, Option<i32>, Option<String>)> =
        dental_clinic::Entity::find()
            // clinic -> city (left join because clinic.city_id is Option<i32>)
            .join(JoinType::LeftJoin, dental_clinic::Relation::City.def())
            // city -> province (left join, but effectively present if city exists)
            .join(JoinType::LeftJoin, city::Relation::Province.def())
            // province -> region (left join)
            .join(JoinType::LeftJoin, province::Relation::Region.def())
            .select_only()
            // clinic id (for mapping)
            .column(dental_clinic::Column::Id)
            // city
            .column(dental_clinic::Column::CityId)
            .column_as(city::Column::Name, "city_name")
            // province
            .column_as(city::Column::ProvinceId, "province_id")
            .column_as(province::Column::Name, "province_name")
            // region
            .column_as(province::Column::RegionId, "region_id")
            .column_as(region::Column::Name, "region_name")
            .into_tuple::<(
                i32,
                Option<i32>,
                Option<String>,
                Option<i32>,
                Option<String>,
                Option<i32>,
                Option<String>,
            )>()
            .all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to load clinic locations: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // 2.6) ✅ ADDED: Index clinic_id -> location info
    #[derive(Clone, Debug)]
    struct Loc {
        _city_id: Option<i32>,
        city_name: Option<String>,
        province_id: Option<i32>,
        province_name: Option<String>,
        region_id: Option<i32>,
        region_name: Option<String>,
    }

    let mut loc_by_clinic: HashMap<i32, Loc> = HashMap::new();
    for (clinic_id, _city_id, city_name, province_id, province_name, region_id, region_name) in clinic_locations {
        loc_by_clinic.insert(
            clinic_id,
            Loc {
                _city_id,
                city_name,
                province_id,
                province_name,
                region_id,
                region_name,
            },
        );
    }




    // 3) Load clinic -> capability_name pairs (only active capabilities)
    let pairs: Vec<(i32, String)> = clinic_capabilities_list::Entity::find()
        .join(
            JoinType::InnerJoin,
            clinic_capabilities_list::Relation::ClinicCapability.def(),
        )
        .filter(clinic_capability::Column::Active.eq(true))
        .select_only()
        .column(clinic_capabilities_list::Column::ClinicId)
        .column_as(clinic_capability::Column::Name, "capability_name")
        .into_tuple::<(i32, String)>()
        .all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load clinic capability pairs: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 4) Index: clinic_id -> set of enabled capability names
    let mut enabled_by_clinic: HashMap<i32, HashSet<String>> = HashMap::new();
    for (clinic_id, capability_name) in pairs {
        enabled_by_clinic
            .entry(clinic_id)
            .or_default()
            .insert(capability_name);
    }

    // 5) Build response
    let mut out: Vec<ClinicWithCapabilities> = Vec::with_capacity(clinics.len());

    for c in clinics {
        let loc = loc_by_clinic.get(&c.id);
        // start all to false
        let mut cap_map: HashMap<String, bool> = all_capability_names
            .iter()
            .map(|name| (name.clone(), false))
            .collect();

        // set enabled to true
        if let Some(enabled) = enabled_by_clinic.get(&c.id) {
            for name in enabled {
                if let Some(v) = cap_map.get_mut(name) {
                    *v = true;
                } else {
                    // In case DB has a capability not in the "active" list (or name changed)
                    cap_map.insert(name.clone(), true);
                }
            }
        }

        out.push(ClinicWithCapabilities {
            id: c.id,
            name: c.name,
            owner_name: c.owner_name,
            address: c.address,
            city_id: c.city_id,
            city_name: loc.and_then(|l| l.city_name.clone()),
            province_id: loc.and_then( |l| l.province_id),
            province_name: loc.and_then(|l|l.province_name.clone()),
            region_id:loc.and_then(|l| l.region_id),
            region_name:loc.and_then(|l| l.region_name.clone()),

            zip_code: c.zip_code,
            remarks: c.remarks,
            contact_numbers: c.contact_numbers,
            email: c.email,
            schedule: c.schedule,
            active: c.active,
            last_modified_by: c.last_modified_by,
            last_modified_on: c.last_modified_on,
            capabilities: cap_map,
        });
    }

    Ok(Json(out))
}
