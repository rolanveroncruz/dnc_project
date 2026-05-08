use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};


use crate::AppState;
use crate::jobs::hmo_billing::generate_hmo_billing_reports;

#[derive(Debug, Serialize)]
pub struct GenerateHmoBillingReportsResponse {
    pub success: bool,
    pub message: String,
}

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