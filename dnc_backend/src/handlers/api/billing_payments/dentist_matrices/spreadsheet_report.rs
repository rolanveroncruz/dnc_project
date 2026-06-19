use axum::body::Body;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::Json;
use http::{header, StatusCode};
use std::io::Cursor;
use std::path::Path;
use tracing::instrument;
use umya_spreadsheet::{reader, writer};
use crate::AppState;
use crate::handlers::api::billing_payments::dentist_matrices::core::{get_dentist_hmo_service_audit_matrix, DentistHmoServiceAuditMatrixQuery, map_report_error};
use crate::handlers::api::billing_payments::dentist_matrices::structs::{DentistHmoAuditDentistRow, DentistHmoServiceAuditMatrixResponse};

// get_cell_total_fee_for_hmo extracts the total fee for a given hmo from a dentist row
fn get_cell_total_fee_for_hmo(
    row: &DentistHmoAuditDentistRow,
    hmo_id: i32,
) -> f64 {
    row.cells
        .iter()
        .find(|cell| cell.hmo_id == hmo_id)
        .map(|cell| cell.cell_total_fee)
        .unwrap_or(0.0)
}

// write_claims_matrix_to_workbook writes the claims matrix to a workbook.
// It takes a report and a workbook as parameters and writes the report to the workbook.
// It returns an error if there is an issue writing the report to the workbook.
fn write_claims_matrix_to_workbook(
    report: &DentistHmoServiceAuditMatrixResponse,
    workbook: &mut umya_spreadsheet::Spreadsheet,
) -> Result<(), String> {

    // 1- Get the first worksheet
    let sheet = workbook
        .get_sheet_mut(&0)
        .ok_or_else(|| "Claims template has no worksheet".to_string())?;

    // 2- Set the header row and first data row
    let header_row: u32 = 6;
    let first_data_row: u32 = 7;

    let fixed_headers = [
        "DENTIST NAME",
        "CONTRACT",
        "PERIOD",
        "CLAIMS",
        "FPS",
        "MFPS/MFPS-2/SFPS",
        "SUBTOTAL",
    ];

    // 3- Write the headers by iterating ove fixed_headers and writing each.
    for (index, header) in fixed_headers.iter().enumerate() {
        let col = (index as u32) + 1;
        sheet
            .get_cell_mut((col, header_row))
            .set_value(*header);
    }
    // 4- Write the HMOs by iterating over report.hmos and writing each.
    for (index, hmo) in report.hmos.iter().enumerate() {
        let col = 8 + index as u32;
        sheet
            .get_cell_mut((col, header_row))
            .set_value(hmo.hmo_short_name.as_str());
    }

    // 5- Write the dentist data by iterating over report.rows and writing each cell.
    for (row_index, dentist_row) in report.rows.iter().enumerate() {
        let excel_row = first_data_row + row_index as u32;

        let fps_fee = if dentist_row.dentist_contract_id == 32 {
            dentist_row.total_basic_fee
        } else {
            0.0
        };

        let mfps_fee = if dentist_row.dentist_contract_id == 32 {
            0.0
        } else {
            dentist_row.total_basic_fee
        };

        // Write Dentist Name
        sheet
            .get_cell_mut((1, excel_row))
            .set_value(dentist_row.dentist_name.as_str());
        // Write Contract
        sheet
            .get_cell_mut((2, excel_row))
            .set_value(dentist_row.dentist_contract_name.as_str());
        // Write Period
        sheet
            .get_cell_mut((3, excel_row))
            .set_value(dentist_row.period.as_str());
        // Write Claims, total_nonbasic_fee
        sheet
            .get_cell_mut((4, excel_row))
            .set_value_number(dentist_row.total_nonbasic_fee);
        // Write FPS,
        sheet
            .get_cell_mut((5, excel_row))
            .set_value_number(fps_fee);
        // Write MFPS,
        sheet
            .get_cell_mut((6, excel_row))
            .set_value_number(mfps_fee);
        // Write Subtotal,
        sheet
            .get_cell_mut((7, excel_row))
            .set_value_number(dentist_row.subtotal_fee);

        // Write HMOs,
        for (hmo_index, hmo) in report.hmos.iter().enumerate() {
            let col = 8 + hmo_index as u32;
            let total_fee = get_cell_total_fee_for_hmo(dentist_row, hmo.hmo_id);

            sheet
                .get_cell_mut((col, excel_row))
                .set_value_number(total_fee);
        }
    }

    Ok(())
}
// get_dentist_hmo_service_audit_matrix_excel_handler()
// is an axum handler that
// returns a spreadsheet report of the dentist hmo service audit matrix.
#[instrument(skip(state))]
pub async fn get_dentist_hmo_service_audit_matrix_excel_handler(
    State(state): State<AppState>,
    Query(query): Query<DentistHmoServiceAuditMatrixQuery>,
) -> Result<Response, (StatusCode, String)> {

    tracing::info!("get_dentist_hmo_service_audit_matrix_excel_handler");
    if query.start_date > query.end_date {
        return Err((
                     StatusCode::BAD_REQUEST,
                     "start_date must be earlier than or equal to end_date".to_string(),
        ));
    }
    // 1- Get the dentist hmo service audit matrix report
    let report = get_dentist_hmo_service_audit_matrix(
                                                       &state.db,
                                                       query.start_date,
                                                       query.end_date,
    )
        .await
        .map_err(map_report_error)?;

    // 2 - Create a workbook from the Claims Template.xlsx file
    let template_path = Path::new("billing_templates/Claims Template.xlsx");

    let mut workbook = reader::xlsx::read(template_path).map_err(|err| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Failed to read Claims Template.xlsx: {err}"),
        )
    })?;
    // 3- Write the report to the workbook
    write_claims_matrix_to_workbook(&report, &mut workbook).map_err(|err| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Failed to write claims matrix workbook: {err}"),
        )
    })?;

    let mut buffer = Cursor::new(Vec::<u8>::new());

    writer::xlsx::write_writer(&workbook, &mut buffer).map_err(|err| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Failed to generate claims matrix xlsx: {err}"),
        )
    })?;

    let filename = format!(
                            "claims_matrix_{}_to_{}.xlsx",
                            query.start_date,
                            query.end_date
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(
                 header::CONTENT_TYPE,
                 "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .header(
                 header::CONTENT_DISPOSITION,
                 format!("attachment; filename=\"{filename}\""),
        )
        .body(Body::from(buffer.into_inner()))
        .map_err(|err| {
            (
              StatusCode::INTERNAL_SERVER_ERROR,
              format!("Failed to build xlsx response: {err}"),
            )
        })
}