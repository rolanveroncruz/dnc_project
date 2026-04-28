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

use crate::AppState;
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
//endregion Helper Functions

// region Create Downloadable XLSX Utilization Report for Company

pub async fn download_utilization_report(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let db: &DatabaseConnection = &state.db;

    let rows = fetch_utilization_report_rows(db, company_id)
        .await
        .map_err(internal_error)?;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet
        .set_name("Utilization Report")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ✅ Headers
    let headers = [
        "Source",
        "ID",
        "Date Created",
        "Dentist ID",
        "Dentist Name",
        "Company ID",
        "Company Name",
        "Member ID",
        "Member Account Number",
        "Member Name",
        "Dental Service Name",
        "Date Service Performed",
        "Tooth",
    ];

    for (col, header_name) in headers.iter().enumerate() {
        worksheet
            .write_string(0, col as u16, *header_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    // ✅ Rows
    for (index, row) in rows.iter().enumerate() {
        let excel_row = (index + 1) as u32;

        worksheet.write_string(excel_row, 0, &row.source)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_number(excel_row, 1, row.id as f64)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 2, &row.date_created.to_string())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_number(excel_row, 3, row.dentist_id as f64)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 4, &row.dentist_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_number(excel_row, 5, row.company_id as f64)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 6, &row.company_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_number(excel_row, 7, row.member_id as f64)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 8, &row.member_account_number)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 9, &row.member_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(excel_row, 10, &row.dental_service_name)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(
            excel_row,
            11,
            &row.date_service_performed
                .map(|d| d.to_string())
                .unwrap_or_default(),
        )
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        worksheet.write_string(
            excel_row,
            12,
            row.tooth.as_deref().unwrap_or(""),
        )
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    // ✅ Optional column widths
    worksheet.set_column_width(0, 16).ok();
    worksheet.set_column_width(2, 24).ok();
    worksheet.set_column_width(4, 30).ok();
    worksheet.set_column_width(6, 30).ok();
    worksheet.set_column_width(8, 22).ok();
    worksheet.set_column_width(9, 30).ok();
    worksheet.set_column_width(10, 30).ok();
    worksheet.set_column_width(11, 22).ok();

    let bytes = workbook
        .save_to_buffer()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let filename = format!("utilization-report-company-{}.xlsx", company_id);

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
