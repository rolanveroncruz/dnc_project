use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    Json,
    response::Response,
};
use sea_orm::{
    DatabaseConnection, DbBackend, Statement, FromQueryResult,
    entity::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{AppState, entities::{endorsement, endorsement_company}};
use std::io::Cursor;
use umya_spreadsheet::{reader, writer, HorizontalAlignmentValues, Style};



// region Get Utilization Report for Company
#[derive(Debug, Deserialize)] // ✅ added query params for date filtering
pub struct UtilizationReportParams {
    pub start_date: Date,
    pub end_date: Date,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct UtilizationReportRow {
    pub source: String,
    pub id: i32,
    pub date_created: DateTimeWithTimeZone,
    pub dentist_id: i32,
    pub dentist_name: String,
    pub company_id: i32,
    pub company_name: String,
    pub member_id: i32,
    pub member_account_number: String,
    pub member_name: String,
    pub dental_service_name: String,
    pub date_service_performed: Option<Date>,
    pub tooth: Option<String>,
}

pub async fn get_utilization_report(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
    Query(params): Query<UtilizationReportParams>,
) -> Result<Json<Vec<UtilizationReportRow>>, (StatusCode, String)> {

    let db: &DatabaseConnection = &state.db;
    let rows = fetch_utilization_report_rows(
        db,
        company_id,
    params.start_date,
    params.end_date)
        .await
        .map_err(internal_error)?;
    Ok(Json(rows))
}

fn internal_error(err: DbErr) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// endregion Get Utilization Report for Company

//region Helper Functions

async fn fetch_utilization_report_rows(
    db: &DatabaseConnection,
    company_id: i32,
    start_date: Date,
    end_date: Date,
) -> Result<Vec<UtilizationReportRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT
            source,
            id,
            date_created,
            dentist_id,
            dentist_name,
            company_id,
            company_name,
            member_id,
            member_account_number,
            member_name,
            dental_service_name,
            date_service_performed,
            tooth
        FROM unified_approved
        WHERE company_id = $1
        AND date_service_performed >= $2
        AND date_service_performed <= $3
        ORDER BY date_service_performed DESC, id DESC
        "#,
        [
            company_id.into(),
            start_date.into(),
            end_date.into(),
        ],
    );

    UtilizationReportRow::find_by_statement(stmt)
        .all(db)
        .await
}

fn create_worksheet_name( company_name: &str)-> String{
    let company_name = company_name.trim();
    if company_name.chars().count()< 15{
        return format!("{} Utilization", company_name);
    }
    let first_word = company_name
        .split_whitespace()
        .next()
        .unwrap_or("");

    if first_word.chars().count() > 15 {
        let shortened_first_word: String = first_word.chars().take(15).collect();
        return format!("{} Utilization", shortened_first_word);
    }
    let mut selected_words:Vec<&str> = Vec::new();
    let mut current_len =0usize;
    for word in company_name.split_whitespace(){
        let word_len = word.chars().count();
        let proposed_len = if selected_words.is_empty(){
            word_len
        } else{
            current_len+1 + word_len
        };
        if proposed_len > 15 {
            break;
        }
        selected_words.push(word);
        current_len = proposed_len;
    }

    format!("{} Utilization", selected_words.join(""))
}
//endregion Helper Functions

// region Create Downloadable XLSX Utilization Report for Company

pub async fn download_utilization_report(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
    Query(params): Query<UtilizationReportParams>,
) -> Result<Response<Body>, (StatusCode, String)> {
    tracing::info!("In download_utilization_report()");

    //-----1. Acquire the database connection and get the company_name from company_id.
    let db: &DatabaseConnection = &state.db;
    let endorsement = endorsement::Entity::find()
        .filter(endorsement::Column::EndorsementCompanyId.eq(company_id))
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Endorsement with Company Id {} not found", company_id),
            )
        })?;

        let agreement_corp_number = endorsement.agreement_corp_number;

    let company = endorsement_company::Entity::find()
        .filter(endorsement_company::Column::Id.eq(company_id))
        .one(db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Company with id {} not found.", company_id),
            )
        })?;

    let company_name = company.name.clone();


    //-----2. Fetch the UtilizationReportRow[]
    let rows = fetch_utilization_report_rows(
        db,
        company_id,
        params.start_date,
        params.end_date)
        .await
        .map_err(internal_error)?;


    //----- 3. Set up the workbook and worksheet.
    let template_path = "billing_templates/Utilization_Report_Template.xlsx";
    let mut book = reader::xlsx::read(template_path)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read XLSX template:{}", e),
            )
        })?;

    // Use the worksheet already present in the template.
    let sheet_name = "Sheet1";

    let worksheet = book
        .get_sheet_by_name_mut(sheet_name)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Worksheet '{}' not found in template", sheet_name),
            )
        })?;

    //----- 3a. Print the company name(agreement_corp_number)
    let top_title = format!(
        "{}({})",
        company_name.clone(),
        agreement_corp_number.unwrap_or_default()
    );
    worksheet
        .get_cell_mut("A1")
        .set_value(top_title);

    //----- 3b. Print the date range
    let date_range_title = format!(
        "Utilization Report for {} - {}",
        params.start_date,
        params.end_date
    );
    worksheet
        .get_cell_mut("A2")
        .set_value(date_range_title);

    //----- 4. Write the rows, skip headers which are already there.
    // Columns are: A - account_number, B - member_name, C - dental_service_name, D - tooth, E- date_service_performed, F - dentist_name
    let first_data_row = 6;
    let mut center_style = Style::default();
    center_style
        .get_alignment_mut()
        .set_horizontal(HorizontalAlignmentValues::Center);

    for (index, row) in rows.iter().enumerate() {
        let excel_row = first_data_row +index as u32;
        worksheet.insert_new_row(&excel_row, &1);
        worksheet
            .get_cell_mut(format!("A{}", excel_row))
            .set_value(row.member_account_number.clone());
        worksheet
            .get_cell_mut(format!("B{}", excel_row))
            .set_value(row.member_name.clone());
        worksheet
            .get_cell_mut(format!("C{}", excel_row))
            .set_value(row.dental_service_name.clone());
        let tooth_cell = format!("D{}", excel_row);
        worksheet
            .get_cell_mut(tooth_cell.as_str())
            .set_value(row.tooth.clone().unwrap_or_default());
        worksheet
            .set_style(tooth_cell, center_style.clone());

        let service_date_cell = format!("E{}", excel_row);
        worksheet
            .get_cell_mut(service_date_cell.as_str())
            .set_value(row.date_service_performed.unwrap().to_string().clone());

        worksheet
            .set_style(service_date_cell, center_style.clone());

        worksheet
            .get_cell_mut(format!("F{}", excel_row))
            .set_value(row.dentist_name.clone());
    }
    //----- 5. Save the workbook to a buffer.
    let mut cursor = Cursor::new(Vec::<u8>::new());

    writer::xlsx::write_writer(&book, &mut cursor)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write XLSX file:{}", e),
            )
        })?;
    let bytes = cursor.into_inner();

    let filename = format!("utilization-report-{}-{}-{}.xlsx", company_name.clone(), params.start_date, params.end_date);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(bytes))
        .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))


}

// endregion Create Downloadable XLSX Utilization Report for Company
