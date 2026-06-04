use tokio::fs;
use std::path::{ Path as FsPath,PathBuf};
use axum::{
    extract::{Path,State},
    Json,
};
use axum::body::Body;
use axum::response::Response;
use http::{header, HeaderMap, HeaderValue, StatusCode};
use tracing::info;
use crate::{
    AppState,
};
use crate::handlers::reports::{get_bill_reports, GeneratedBillingReportResponse};

// get_generated_hmo_billing_reports returns a list of generated billing reports based on
// the table
pub async fn get_generated_hmo_billing_reports(
    State(state): State<AppState>,
) -> Result<Json<Vec<GeneratedBillingReportResponse>>, StatusCode> {
    let reports = get_bill_reports(&state.db, 1)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reports))
}


pub async fn download_generated_report(
    Path(file_name): Path<String>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // ✅ Folder where your generated reports are stored
    let reports_dir = PathBuf::from("../../../../generated_reports");

    // ✅ Prevent ../../ attacks
    let safe_file_name = FsPath::new(&file_name)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid file name".to_string(),
            )
        })?;

    // ✅ Full file path becomes: generated_reports/<file_name>
    let file_path = reports_dir.join(safe_file_name);
    info!(target:"jobs", "downloading file from {}", file_path.display());
    
    // ✅ Read the file from generated_reports/<file_name>
    let bytes = fs::read(&file_path)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                format!("File not found: {}", safe_file_name),
            )
        })?;

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}\"",
            safe_file_name
        ))
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid file name for download".to_string(),
                )
            })?,
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(bytes))
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build response".to_string(),
            )
        })?)
}