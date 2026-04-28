use axum::{
    body::Body,
    extract::{Path, State},
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
use rust_xlsxwriter::Workbook;
#[derive(Debug, Serialize, Deserialize, FromQueryResult)]

// region Get Utilization Report for Company
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
) -> Result<Json<Vec<UtilizationReportRow>>, (StatusCode, String)> {

    let db: &DatabaseConnection = &state.db;
    let rows = fetch_utilization_report_rows(db, company_id)
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
) -> Result<Vec<UtilizationReportRow>, sea_orm::DbErr> {
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
        ORDER BY date_created DESC, id DESC
        "#,
        [company_id.into()],
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
    let rows = fetch_utilization_report_rows(db, company_id)
        .await
        .map_err(internal_error)?;


    //----- 3. Setup the workbook and worksheet.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let worksheet_name = create_worksheet_name(&company_name);

    worksheet
        .set_name(worksheet_name.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    //----3a. Print the company_name(agreement_corp_number) at 0,0
    let top_title =format!("{}({})", company_name.clone(), agreement_corp_number.unwrap());
    worksheet
        .write_string(0,0, top_title)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    //-----4. Write the headers.
    // ✅ Headers
    let headers = [
        "CERT NUMBER",
        "PATIENT NAME",
        "TREATMENT DONE",
        "TEETH NUMBER",
        "TREATMENT DATE",
        "DENTIST NAME",
        "AMOUNT",
    ];
    let header_row= 4;
    for (col, header_name) in headers.iter().enumerate() {
        worksheet
            .write_string(header_row, col as u16, *header_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    //-----5. Write the rows
    // ✅ Rows
    for (index, row) in rows.iter().enumerate() {
        let excel_row = (index + ((header_row as usize) +1))  as u32;

        worksheet.write_string(excel_row, 0, &row.member_account_number)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 1, &row.member_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 2, &row.dental_service_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 3, row.tooth.clone().as_deref().unwrap_or(" "),)
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

         let treatment_date = row.
             date_service_performed
             .map(|d| d.to_string())
             .unwrap_or("".to_string());
        worksheet.write_string(excel_row, 4,  treatment_date)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 5, &row.dentist_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_number(excel_row, 6, 500.00)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    }

    // ✅ Optional column widths
    worksheet.set_column_width(0, 16).ok();
    worksheet.set_column_width(1, 24).ok();
    worksheet.set_column_width(2, 30).ok();
    worksheet.set_column_width(3, 30).ok();
    worksheet.set_column_width(4, 22).ok();
    worksheet.set_column_width(5, 30).ok();
    worksheet.set_column_width(6, 30).ok();

    let bytes = workbook
        .save_to_buffer()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let filename = format!("util-z-rpt-{}.xlsx", company_name.clone());

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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// endregion Create Downloadable XLSX Utilization Report for Company
