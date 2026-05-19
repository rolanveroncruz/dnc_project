/*
 This exposes test_generate_hmo_billing_reports to be used as an API end point for testing purposes.
 */
use axum::{
    extract::{State},
    Json,
};
use chrono::{Utc};
use serde::{Serialize};


use crate::AppState;
use crate::jobs::hmo_billing::generate_hmo_billing_reports;

#[derive(Debug, Serialize)]
pub struct GenerateHmoBillingReportsResponse {
    pub success: bool,
    pub message: String,
}
// test_generate_hmo_billing_reports is an API endpoint that generates an HMO billing report.
// it doesn't accept any parameters.
pub async fn test_generate_hmo_billing_reports(
    State(state): State<AppState>,
) -> Result<Json<GenerateHmoBillingReportsResponse>, (axum::http::StatusCode, String)> {
    let today = Utc::now().date_naive();
    generate_hmo_billing_reports(
        state,
        None,
        today
    )
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate HMO billing reports: {}", e),
            )
        })?;

    Ok(Json(GenerateHmoBillingReportsResponse {
        success: true,
        message: "HMO billing reports generated successfully".to_string(),
    }))
}