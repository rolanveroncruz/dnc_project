use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::entity::prelude::Date;
use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::entities::{
    dental_service,
    endorsement,
    endorsement_company,
    endorsement_counts,
    hmo,
};
use crate::AppState;


// ============================================================
// Response structs
// ============================================================

#[derive(Debug, Clone, Serialize)]
pub struct CsrEndorsementResponse {
    pub id: i32,
    pub hmo_short_name: String,
    pub company_name: String,
    pub date_start: Date,
    pub date_end: Date,
    pub benefits: String,
}


// ============================================================
// Dental benefits helper
// ============================================================

/// Builds the dental-benefit description for multiple endorsements.
///
/// Every endorsement starts with:
///
///     Basic
///
/// Non-basic dental services having a count > 0 are then appended:
///
///     Basic (+) Oral Prophylaxis (1) (+) Extraction (2)
///
async fn get_dental_benefits_strings(
    db: &DatabaseConnection,
    endorsement_ids: &[i32],
) -> Result<HashMap<i32, String>, DbErr> {
    let mut benefits_by_endorsement: HashMap<i32, String> = endorsement_ids
        .iter()
        .copied()
        .map(|id| (id, String::from("Basic")))
        .collect();

    if endorsement_ids.is_empty() {
        return Ok(benefits_by_endorsement);
    }

    let benefit_rows = endorsement_counts::Entity::find()
        .find_also_related(dental_service::Entity)
        .filter(
            endorsement_counts::Column::EndorsementId
                .is_in(endorsement_ids.iter().copied()),
        )
        .filter(endorsement_counts::Column::Count.gt(0))
        // Exclude basic services.
        //
        // Basic services are represented by the initial "Basic" string.
        .filter(dental_service::Column::TypeId.ne(1))
        .order_by_asc(endorsement_counts::Column::EndorsementId)
        .order_by_asc(dental_service::Column::SortIndex)
        .order_by_asc(dental_service::Column::Name)
        .all(db)
        .await?;

    for (count_row, service) in benefit_rows {
        let Some(service) = service else {
            continue;
        };

        let benefits = benefits_by_endorsement
            .entry(count_row.endorsement_id)
            .or_insert_with(|| String::from("Basic"));

        benefits.push_str(
            &format!(
                " (+) {} ({})",
                service.name,
                count_row.count,
            ),
        );
    }

    Ok(benefits_by_endorsement)
}


// ============================================================
// GET endorsements for CSRs
// ============================================================

pub async fn get_endorsements_for_csr(
    State(state): State<AppState>,
) -> Result<Json<Vec<CsrEndorsementResponse>>, (StatusCode, String)> {
    let db = &state.db;

    // --------------------------------------------------------
    // 1. Get endorsements
    // --------------------------------------------------------

    let endorsements = endorsement::Entity::find()
        .order_by_desc(endorsement::Column::DateStart)
        .order_by_desc(endorsement::Column::Id)
        .all(db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load endorsements: {err}"),
            )
        })?;

    if endorsements.is_empty() {
        return Ok(Json(Vec::new()));
    }


    // --------------------------------------------------------
    // 2. Collect IDs needed for the related records
    // --------------------------------------------------------

    let endorsement_ids: Vec<i32> = endorsements
        .iter()
        .map(|row| row.id)
        .collect();

    let hmo_ids: Vec<i32> = endorsements
        .iter()
        .map(|row| row.hmo_id)
        .collect();

    let company_ids: Vec<i32> = endorsements
        .iter()
        .map(|row| row.endorsement_company_id)
        .collect();


    // --------------------------------------------------------
    // 3. Load HMOs
    // --------------------------------------------------------

    let hmos = hmo::Entity::find()
        .filter(hmo::Column::Id.is_in(hmo_ids))
        .all(db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load HMOs: {err}"),
            )
        })?;

    let hmo_name_by_id: HashMap<i32, String> = hmos
        .into_iter()
        .map(|row| (row.id, row.short_name))
        .collect();


    // --------------------------------------------------------
    // 4. Load companies
    // --------------------------------------------------------

    let companies = endorsement_company::Entity::find()
        .filter(endorsement_company::Column::Id.is_in(company_ids))
        .all(db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load endorsement companies: {err}"),
            )
        })?;

    let company_name_by_id: HashMap<i32, String> = companies
        .into_iter()
        .map(|row| (row.id, row.name))
        .collect();


    // --------------------------------------------------------
    // 5. Generate benefit strings
    // --------------------------------------------------------

    let benefits_by_endorsement =
        get_dental_benefits_strings(db, &endorsement_ids)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to load endorsement benefits: {err}"),
                )
            })?;


    // --------------------------------------------------------
    // 6. Build response
    // --------------------------------------------------------

    let mut response = Vec::with_capacity(endorsements.len());

    for row in endorsements {
        let hmo_short_name = hmo_name_by_id
            .get(&row.hmo_id)
            .cloned()
            .unwrap_or_default();

        let company_name = company_name_by_id
            .get(&row.endorsement_company_id)
            .cloned()
            .unwrap_or_default();

        let benefits = benefits_by_endorsement
            .get(&row.id)
            .cloned()
            .unwrap_or_else(|| String::from("Basic"));

        response.push(CsrEndorsementResponse {
            id: row.id,
            hmo_short_name,
            company_name,
            date_start: row.date_start,
            date_end: row.date_end,
            benefits,
        });
    }

    Ok(Json(response))
}