use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType , PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::AppState;
use crate::entities::{city, dental_clinic, province, region,
                      clinic_capability, clinic_capabilities_list };

use std::collections::HashMap;

//
// ---- List response (paging)
//

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,      // 1-based
    pub page_size: Option<u64>, // clamp server-side
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub page: u64, // 1-based
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct DentalClinicListQuery {
    #[serde(flatten)]
    pub base: ListQuery,

    // Optional filters
    pub city_id: Option<i32>,
    pub active: Option<bool>,
    pub name_like: Option<String>,
}
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentalClinicRowDb {
    // ---- dental_clinic columns
    pub id: i32,
    pub name: String,
    pub owner_name: Option<String>,
    pub address: String,
    pub city_id: Option<i32>,
    pub zip_code: Option<String>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub email: Option<String>,
    pub schedule: Option<String>,
    pub active: Option<bool>,
    pub last_modified_by: String,
    pub last_modified_on: DateTimeWithTimeZone,

    // ---- joined fields
    pub city_name: Option<String>,
    pub province_id: Option<i32>,
    pub province_name: Option<String>,
    pub region_id: Option<i32>,
    pub region_name: Option<String>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentalClinicRow {
    // ---- dental_clinic columns
    pub id: i32,
    pub name: String,
    pub owner_name: Option<String>,
    pub address: String,
    pub city_id: Option<i32>,
    pub zip_code: Option<String>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub email: Option<String>,
    pub schedule: Option<String>,
    pub active: Option<bool>,
    pub last_modified_by: String,
    pub last_modified_on: DateTimeWithTimeZone,

    // ---- joined fields
    pub city_name: Option<String>,
    pub province_id: Option<i32>,
    pub province_name: Option<String>,
    pub region_id: Option<i32>,
    pub region_name: Option<String>,

    //---- clinic capabilities
    #[serde(rename="hasPanoramic")]
    pub has_panoramic: bool,

    #[serde(rename="hasPeriapical")]
    pub has_periapical: bool,
}
impl From<DentalClinicRowDb> for DentalClinicRow {
    fn from(db: DentalClinicRowDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            owner_name: db.owner_name,
            address: db.address,
            city_id: db.city_id,
            zip_code: db.zip_code,
            remarks: db.remarks,
            contact_numbers: db.contact_numbers,
            email: db.email,
            schedule: db.schedule,
            active: db.active,
            last_modified_by: db.last_modified_by,
            last_modified_on: db.last_modified_on,
            city_name: db.city_name,
            province_id: db.province_id,
            province_name: db.province_name,
            region_id: db.region_id,
            region_name: db.region_name,
            has_panoramic: false,
            has_periapical: false,
        }
    }
}
#[derive(Debug, FromQueryResult)]
struct ClinicCapRow {
    pub clinic_id: i32,
    pub capability_name: String,
}

//////////////
//
// get_dental_clinics
//
//////////////

#[instrument(skip(state), err(Debug))]
pub async fn get_dental_clinics(
    State(state): State<AppState>,
    Query(params): Query<DentalClinicListQuery>,
) -> Result<Json<PageResponse<DentalClinicRow>>, StatusCode> {
    let page = params.base.page.unwrap_or(1).clamp(1, u64::MAX);
    let page_size = params.base.page_size.unwrap_or(650).clamp(1, 1000);
    let page0 = page.saturating_sub(1);

    let mut q = dental_clinic::Entity::find()
        .join(JoinType::LeftJoin, dental_clinic::Relation::City.def())
        .join(JoinType::LeftJoin, city::Relation::Province.def())
        .join(JoinType::LeftJoin, province::Relation::Region.def())
        .order_by_asc(dental_clinic::Column::Name);

    if let Some(city_id) = params.city_id {
        q = q.filter(dental_clinic::Column::CityId.eq(city_id));
    }
    if let Some(active) = params.active {
        q = q.filter(dental_clinic::Column::Active.eq(active));
    }
    if let Some(name_like) = params
        .name_like
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        q = q.filter(dental_clinic::Column::Name.contains(name_like));
    }

    let q = q
        .select_only()
        .columns([
            dental_clinic::Column::Id,
            dental_clinic::Column::Name,
            dental_clinic::Column::OwnerName,
            dental_clinic::Column::Address,
            dental_clinic::Column::CityId,
            dental_clinic::Column::ZipCode,
            dental_clinic::Column::Remarks,
            dental_clinic::Column::ContactNumbers,
            dental_clinic::Column::Email,
            dental_clinic::Column::Schedule,
            dental_clinic::Column::Active,
            dental_clinic::Column::LastModifiedBy,
            dental_clinic::Column::LastModifiedOn,
        ])
        .column_as(city::Column::Name, "city_name")
        .column_as(province::Column::Id, "province_id")
        .column_as(province::Column::Name, "province_name")
        .column_as(province::Column::RegionId, "region_id")
        .column_as(region::Column::Name, "region_name")

        .order_by_asc(dental_clinic::Column::Name)
        .into_model::<DentalClinicRowDb>();

    let paginator = q.paginate(&state.db, page_size);

    let total = paginator.num_items().await.map_err(|e| {
        tracing::error!("Failed to count dental clinics: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items_db = paginator.fetch_page(page0).await.map_err(|e| {
        tracing::error!("Failed to fetch dental clinics page={page} size={page_size}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // ---- build initial API rows (default flags = false)
    let mut items: Vec<DentalClinicRow> = items_db.into_iter().map(Into::into).collect();

    // ---- fetch capabilities for only these clinics
    let clinic_ids: Vec<i32> = items.iter().map(|r| r.id).collect();

    if !clinic_ids.is_empty() {
        let caps = clinic_capabilities_list::Entity::find()
            .join(
                JoinType::InnerJoin,
                clinic_capabilities_list::Relation::ClinicCapability.def(),
            )
            .filter(clinic_capabilities_list::Column::ClinicId.is_in(clinic_ids.clone()))
            .filter(
                clinic_capability::Column::Name.is_in([
                    "Dental Radiography (Panoramic)".to_string(),
                    "Dental Radiography (Periapical)".to_string(),
                ]),
            )
            .select_only()
            .column(clinic_capabilities_list::Column::ClinicId)
            .column_as(clinic_capability::Column::Name, "capability_name")
            .into_model::<ClinicCapRow>()
            .all(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch clinic capabilities for page: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        // Map: clinic_id -> (has_panoramic, has_periapical)
        let mut flags: HashMap<i32, (bool, bool)> = HashMap::new();
        for c in caps {
            let entry = flags.entry(c.clinic_id).or_insert((false, false));
            match c.capability_name.as_str() {
                "Dental Radiography (Panoramic)" => entry.0 = true,
                "Dental Radiography (Periapical)" => entry.1 = true,
                _ => {}
            }
        }

        // Apply flags
        for r in &mut items {
            if let Some((pano, peri)) = flags.get(&r.id) {
                r.has_panoramic = *pano;
                r.has_periapical = *peri;
            }
        }
    }



    Ok(Json(PageResponse {
        page,
        page_size,
        total,
        items,
    }))
}

//////////////
//
// get_dental_clinic_by_id
//
//////////////

#[instrument(skip(state), err(Debug))]
pub async fn get_dental_clinic_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<DentalClinicRowDb>, StatusCode> {
    let row = dental_clinic::Entity::find()
        .filter(dental_clinic::Column::Id.eq(id))
        .join(JoinType::LeftJoin, dental_clinic::Relation::City.def())
        .join(JoinType::LeftJoin, city::Relation::Province.def())
        .join(JoinType::LeftJoin, province::Relation::Region.def())
        .select_only()
        .columns([
            dental_clinic::Column::Id,
            dental_clinic::Column::Name,
            dental_clinic::Column::OwnerName,
            dental_clinic::Column::Address,
            dental_clinic::Column::CityId,
            dental_clinic::Column::ZipCode,
            dental_clinic::Column::Remarks,
            dental_clinic::Column::ContactNumbers,
            dental_clinic::Column::Email,
            dental_clinic::Column::Schedule,
            dental_clinic::Column::Active,
            dental_clinic::Column::LastModifiedBy,
            dental_clinic::Column::LastModifiedOn,
        ])
        .column_as(city::Column::Name, "city_name")
        .column_as(province::Column::Id, "province_id")
        .column_as(province::Column::Name, "province_name")
        .column_as(province::Column::RegionId, "region_id")
        .column_as(region::Column::Name, "region_name")
        .into_model::<DentalClinicRowDb>()
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dental clinic {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match row {
        Some(model) => Ok(Json(model)),
        None => Err(StatusCode::NOT_FOUND),
    }
}




//
// ---- POST: create clinic
//

#[derive(Debug, Deserialize)]
pub struct CreateDentalClinicBody {
    pub name: String,
    pub address: String,
    pub owner_name: Option<String>,
    pub city_id: Option<i32>,
    pub zip_code: Option<String>,
    pub remarks: Option<String>,
    pub contact_numbers: Option<String>,
    pub email: Option<String>,
    pub schedule: Option<String>,
    pub active: Option<bool>,
    pub last_modified_by: String,
}

#[instrument(skip(state), err(Debug))]
pub async fn create_dental_clinic(
    State(state): State<AppState>,
    Json(body): Json<CreateDentalClinicBody>,
) -> Result<Json<dental_clinic::Model>, StatusCode> {
    let name = body.name.trim();
    let address = body.address.trim();

    if name.is_empty() || address.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Optional: application-level uniqueness guard (name+address+city_id+zip_code)
    let mut dupe_q = dental_clinic::Entity::find()
        .filter(dental_clinic::Column::Name.eq(name))
        .filter(dental_clinic::Column::Address.eq(address));

    match body.city_id {
        Some(cid) => dupe_q = dupe_q.filter(dental_clinic::Column::CityId.eq(cid)),
        None => dupe_q = dupe_q.filter(dental_clinic::Column::CityId.is_null()),
    }

    match body
        .zip_code
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        Some(z) => dupe_q = dupe_q.filter(dental_clinic::Column::ZipCode.eq(z)),
        None => dupe_q = dupe_q.filter(dental_clinic::Column::ZipCode.is_null()),
    }

    let dupe = dupe_q.one(&state.db).await.map_err(|e| {
        tracing::error!("Failed to check duplicate dental clinic: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if dupe.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = Utc::now().into();

    let am = dental_clinic::ActiveModel {
        name: Set(name.to_string()),
        owner_name: Set(body.owner_name),
        address: Set(address.to_string()),
        city_id: Set(body.city_id),
        zip_code: Set(body
            .zip_code
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())),
        remarks: Set(body.remarks),
        contact_numbers: Set(body.contact_numbers),
        email: Set(body.email),
        schedule: Set(body.schedule),
        active: Set(body.active),
        last_modified_by: Set(body.last_modified_by),
        last_modified_on: Set(now),
        ..Default::default()
    };

    let created = am.insert(&state.db).await.map_err(|e| {
        tracing::error!("Failed to create dental clinic: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(created))
}

//
// ---- PATCH: partial update
//

#[derive(Debug, Deserialize)]
pub struct PatchDentalClinicBody {
    pub name: Option<String>,
    pub owner_name: Option<Option<String>>,
    pub address: Option<String>,
    pub city_id: Option<Option<i32>>, // Some(None)=explicitly null it; None=don't change
    pub zip_code: Option<Option<String>>,
    pub remarks: Option<Option<String>>,
    pub contact_numbers: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub schedule: Option<Option<String>>,
    pub active: Option<Option<bool>>,
    pub last_modified_by: String,
}

#[instrument(skip(state), err(Debug))]
pub async fn patch_dental_clinic(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<PatchDentalClinicBody>,
) -> Result<Json<dental_clinic::Model>, StatusCode> {
    let existing = dental_clinic::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch dental clinic {id}: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let Some(existing) = existing else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut am: dental_clinic::ActiveModel = existing.into();

    if let Some(name) = body.name {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        am.name = Set(name);
    }
    if let Some(owner_name) = body.owner_name {
        am.owner_name = Set(owner_name.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()));
    }

    if let Some(address) = body.address {
        let address = address.trim().to_string();
        if address.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }
        am.address = Set(address);
    }

    if let Some(city_id) = body.city_id {
        am.city_id = Set(city_id);
    }

    if let Some(zip_code) = body.zip_code {
        am.zip_code = Set(zip_code
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()));
    }

    if let Some(v) = body.remarks {
        am.remarks = Set(v);
    }
    if let Some(v) = body.contact_numbers {
        am.contact_numbers = Set(v);
    }
    if let Some(v) = body.email {
        am.email = Set(v);
    }
    if let Some(v) = body.schedule {
        am.schedule = Set(v);
    }
    if let Some(v) = body.active {
        am.active = Set(v);
    }

    am.last_modified_by = Set(body.last_modified_by);
    am.last_modified_on = Set(Utc::now().into());

    let updated = am.update(&state.db).await.map_err(|e| {
        tracing::error!("Failed to patch dental clinic {id}: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated))
}
