use axum::{extract::{State}, http::{StatusCode}, Json};
use sea_orm::{
    DatabaseBackend,  FromQueryResult, Statement,
};
use serde::Serialize;

use crate::AppState;

// region: Get Dentist Clinics Reconciled Jobs Count Last 12 Months
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistClinicReconciledJobsCountLast12MonthsRow {
    pub id: Option<i32>,
    pub dentist_name: Option<String>,
    pub clinic_name: Option<String>,
    pub position_name: Option<String>,
    pub contract_name: Option<String>,

    // timestamp without time zone
    pub month_start: Option<chrono::NaiveDateTime>,

    pub month_label: Option<String>,

    // PostgreSQL bigint -> i64
    pub rec_services_count: Option<i64>,
}

pub async fn get_dentist_clinics_reconciled_jobs_count_last_12_months(
    State(state): State<AppState>,
) -> Result<Json<Vec<DentistClinicReconciledJobsCountLast12MonthsRow>>, (StatusCode, String)> {
    let rows = DentistClinicReconciledJobsCountLast12MonthsRow::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            SELECT
                id,
                dentist_name,
                clinic_name,
                position_name,
                contract_name,
                month_start,
                month_label,
                rec_services_count
            FROM dentist_clinics_reconciled_jobs_count_last_12_months
            WHERE rec_services_count > 0
            ORDER BY
                dentist_name ASC,
                clinic_name ASC,
                month_start ASC
            "#,
            [],
        ),
    )
        .all(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error{:?}", err),
            )
        })?;
    Ok(Json(rows))
}

// endregion: Get Dentist Clinics Reconciled Jobs Count Last 12 Months
