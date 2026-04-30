use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    Json,
    response::Response,
};

use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{ Serialize, Deserialize };
use crate::{
    AppState,
    entities::{
        endorsement,
        endorsement_company,
        endorsement_billing_period_type,
        master_list_member,
    },
};
// region Get HMO Billing

// ✅ Add this near your other response structs
#[derive(Debug, Serialize, Deserialize)]
pub struct HMOBillingRow {
    pub statement_of_account_no: String,
    pub company_name: String,
    pub agreement_corp_number: Option<String>,
    pub total_master_list_members: u64,
    pub billing_period_type_name: String,
    pub dental_benefits: String,
    pub effectivity_period: String,
    pub retainer_fee: Option<String>,
}


// ✅ GET /api/v1/hmo-billing/:hmo_id
pub async fn get_hmo_billing(
    State(state): State<AppState>,
    Path(hmo_id): Path<i32>,
) -> Result<Json<Vec<HMOBillingRow>>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    let items = get_hmo_billing_rows(db, hmo_id).await?;
    Ok(Json(items))
}

// endregion Get HMO Billing

// region Get HMO Billing Helper functions
fn compute_dental_benefits(endorsement_type_id: i32) -> String {
    match endorsement_type_id {
        1 => "Basic Services".to_string(),
        2 => "Basic Services + Special Services".to_string(),
        3 => "Basic Services + Special Services + High-End Services".to_string(),
        _ => "Dental Benefits".to_string(),
    }
}

fn format_billing_period_type_name(name: &str) -> String {
    let mut parts = name.split_whitespace();

    match (parts.next(), parts.next()) {
        (Some("Billed"), Some(second_word)) => second_word.to_string(),
        _ => name.to_string(),
    }
}

// endregion Get HMO Billing Helper functions


// region Download HMO Billing

// ✅ GET /api/v1/hmo-billing/:hmo_id/download
pub async fn download_hmo_billing(
    State(state): State<AppState>,
    Path(hmo_id): Path<i32>,
) -> Result<Response, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;
    tracing::info!("download_hmo_billing {}", hmo_id);

    // ✅ 1. Get data
    let rows = get_hmo_billing_rows(db, hmo_id).await?;

    // ✅ 2. Load existing template XLSX
    let template_path = "billing_templates/HMO_Billing_Template.xlsx";

    let mut book = umya_spreadsheet::reader::xlsx::read(template_path)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read XLSX template: {}", e),
            )
        })?;

    // ✅ 3. Pick worksheet
    // Change "Sheet1" to the actual worksheet name in your template.
    let sheet = book
        .get_sheet_by_name_mut("summary")
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Worksheet 'Sheet1' not found in template".to_string(),
            )
        })?;

    // ✅ 4. Optional report title / metadata cells
    sheet.get_cell_mut("A1").set_value("HMO Billing");

    // ✅ 5. Write data rows
    //
    // Columns assumed:
    // A = Company Name
    // B = Agreement Corp Number
    // C = Total Master List Members
    // D = Billing Period Type
    // E = Dental Benefits
    // F = Effectivity Period
    // G = Retainer Fee
    let start_row: u32 = 8;

    for (index, row) in rows.iter().enumerate() {
        let excel_row = start_row + index as u32;

        sheet
            .get_cell_mut(format!("A{}", excel_row))
            .set_value(row.company_name.clone());

        sheet
            .get_cell_mut(format!("B{}", excel_row))
            .set_value(row.agreement_corp_number.clone().unwrap_or_default());

        sheet
            .get_cell_mut(format!("C{}", excel_row))
            .set_value(row.total_master_list_members.to_string());

        sheet
            .get_cell_mut(format!("D{}", excel_row))
            .set_value(row.billing_period_type_name.clone());

        sheet
            .get_cell_mut(format!("E{}", excel_row))
            .set_value(row.dental_benefits.clone());

        sheet
            .get_cell_mut(format!("F{}", excel_row))
            .set_value(row.effectivity_period.clone());

        sheet
            .get_cell_mut(format!("G{}", excel_row))
            .set_value(row.retainer_fee.clone().unwrap_or_default());
    }

    // ✅ 6. Write the workbook directly to bytes
    let mut buffer : Vec<u8> = Vec::new();

    umya_spreadsheet::writer::xlsx::write_writer(&book, &mut buffer)
        .map_err(|e|{
            (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write XLSX output: {}", e),
            )
    })?;

    // ✅ 7. Return as downloadable XLSX
    let filename = format!("hmo_billing_{}.xlsx", hmo_id);

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                )
            })?,
    );

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(buffer))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            )
        })?;

    let mut response = response;

    for (key, value) in headers {
        if let Some(key) = key {
            response.headers_mut().insert(key, value);
        }
    }

    Ok(response)
}


// endregion Download HMO Billing

// region Download HMO Billing Helper Functions
// ✅ Shared query helper
async fn get_hmo_billing_rows(
    db: &DatabaseConnection,
    hmo_id: i32,
) -> Result<Vec<HMOBillingRow>, (StatusCode, String)> {
    let endorsements = endorsement::Entity::find()
        .filter(endorsement::Column::HmoId.eq(hmo_id))
        .filter(endorsement::Column::IsActive.eq(true))
        .all(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut items: Vec<HMOBillingRow> = Vec::new();

    for endorsement_row in endorsements {
        let company = endorsement_company::Entity::find_by_id(
            endorsement_row.endorsement_company_id,
        )
            .one(db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Endorsement company not found: {}",
                        endorsement_row.endorsement_company_id
                    ),
                )
            })?;

        let billing_period_type =
            endorsement_billing_period_type::Entity::find_by_id(
                endorsement_row.endorsement_billing_period_type_id,
            )
                .one(db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .ok_or_else(|| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "Billing period type not found: {}",
                            endorsement_row.endorsement_billing_period_type_id
                        ),
                    )
                })?;

        let total_master_list_members = master_list_member::Entity::find()
            .filter(master_list_member::Column::EndorsementId.eq(endorsement_row.id))
            .count(db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let effectivity_period = format!(
            "{} - {}",
            endorsement_row.date_start,
            endorsement_row.date_end
        );

        items.push(HMOBillingRow {
            statement_of_account_no: "XXX-XXXX".to_string(),
            company_name: company.name,
            agreement_corp_number: endorsement_row.agreement_corp_number,
            total_master_list_members,
            billing_period_type_name: format_billing_period_type_name(
                &billing_period_type.name,
            ),
            dental_benefits: compute_dental_benefits(
                endorsement_row.endorsement_type_id,
            ),
            effectivity_period,
            retainer_fee: endorsement_row.retainer_fee.map(|v| v.to_string()),
        });
    }

    Ok(items)
}

// endregion Download HMO Billing Helper Functions
