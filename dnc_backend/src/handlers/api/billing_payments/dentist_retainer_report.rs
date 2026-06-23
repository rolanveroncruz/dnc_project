/*
This file deals with the Dentist Retainer report.
 */
use axum::{
        extract::{Query, State},
        http::StatusCode,
        Json
};
use chrono::NaiveDate;
use chrono::Datelike;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};
use crate::AppState;

// region: Structures
// The request details.
#[derive(Debug, Clone, Deserialize)]
pub struct DentistRetainerPayablesQuery {
    pub year: i32,
    pub month: u32,
}

// The response details.
#[derive(Debug, Clone, Serialize)]
pub struct DentistRetainerPayablesResponse {
    pub year: i32,
    pub month: u32,
    pub month_label: String,

    pub rows: Vec<DentistRetainerPayableRow>,

    pub grand_total_rate: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistRetainerPayableRow {
    pub dentist_clinic_id: i32,

    pub dentist_id: i32,
    pub dentist_name: String,

    pub clinic_id: i32,
    pub clinic_name: String,

    pub retainer_fee: f64,

    /// retainer_fee * number of included months
    pub rate: f64,

    /// Example: "April 2026, May 2026"
    pub remarks: String,

    /// Latest date_service_performed among included months
    pub date_of_latest: Option<NaiveDate>,
}
// Structure from the database??
#[derive(Debug, FromQueryResult)]
pub struct DentistRetainerPayableSourceRow {
    pub dentist_clinic_id: i32,

    pub dentist_id: i32,
    pub dentist_name: String,

    pub clinic_id: i32,
    pub clinic_name: String,

    pub retainer_fee: f64,

    pub rate: f64,
    pub remarks: Option<String>,
    pub date_of_latest: Option<NaiveDate>,
}

#[derive(Debug, thiserror::Error)]
pub enum DentistRetainerPayablesError {
    #[error("Invalid month. Month must be between 1 and 12.")]
    InvalidMonth,

    #[error("Invalid year/month combination.")]
    InvalidDate,

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}
// endregion: Structures

// region: get_dentist_retainer_payables
//
// get_dentist_retainer_payables from a Query { year, month}, returns
// a response {year, month, rows of DentistRetainerPayableRow {dentist, dental_clinic, rate, remarks}.
pub async fn get_dentist_retainer_payables(
    db: &DatabaseConnection,
    query: DentistRetainerPayablesQuery,
) -> Result<DentistRetainerPayablesResponse, DentistRetainerPayablesError> {

    if query.month < 1 || query.month > 12 {
        return Err(DentistRetainerPayablesError::InvalidMonth);
    }

    let selected_month_start = NaiveDate::from_ymd_opt(query.year, query.month, 1)
        .ok_or(DentistRetainerPayablesError::InvalidDate)?;

    let selected_next_month_start = add_one_month(selected_month_start)?;

    let month_label = format_month_year(query.year, query.month);

    let sql = r#"
        WITH principal_flat_fee AS (
            SELECT
                dentist_clinic.id AS dentist_clinic_id,

                dentist.id AS dentist_id,
                CONCAT_WS(
                    ' ',
                    CONCAT(dentist.last_name, ','),
                    dentist.given_name,
                    dentist.middle_name
                ) AS dentist_name,

                dental_clinic.id AS clinic_id,
                dental_clinic.name AS clinic_name,

                dentist.retainer_fee::float8 AS retainer_fee
            FROM dentist_clinic
            INNER JOIN dentist
                ON dentist.id = dentist_clinic.dentist_id
            INNER JOIN dental_clinic
                ON dental_clinic.id = dentist_clinic.clinic_id
            WHERE dentist_clinic.position_id = 1
              AND dentist.accre_dentist_contract_id = 1
        ),

        monthly_activity AS (
            SELECT
                verification.dentist_id,
                verification.dental_clinic_id AS clinic_id,

                EXTRACT(YEAR FROM verification.date_service_performed)::int AS service_year,
                EXTRACT(MONTH FROM verification.date_service_performed)::int AS service_month,

                MAX(verification.date_service_performed) AS latest_date_service_performed
            FROM verification
            INNER JOIN principal_flat_fee
                ON principal_flat_fee.dentist_id = verification.dentist_id
               AND principal_flat_fee.clinic_id = verification.dental_clinic_id
            WHERE verification.date_service_performed IS NOT NULL
              AND verification.date_service_performed < $3
            GROUP BY
                verification.dentist_id,
                verification.dental_clinic_id,
                EXTRACT(YEAR FROM verification.date_service_performed)::int,
                EXTRACT(MONTH FROM verification.date_service_performed)::int
        )

        SELECT
            principal_flat_fee.dentist_clinic_id,

            principal_flat_fee.dentist_id,
            principal_flat_fee.dentist_name,

            principal_flat_fee.clinic_id,
            principal_flat_fee.clinic_name,

            principal_flat_fee.retainer_fee,

            (
                principal_flat_fee.retainer_fee *
                COUNT(monthly_activity.*) FILTER (
                    WHERE
                        monthly_activity.dentist_id IS NOT NULL
                        AND (
                            (
                                monthly_activity.service_year = $1
                                AND monthly_activity.service_month = $2
                            )
                            OR dentist_payments.id IS NULL
                        )
                )::float8
            ) AS rate,

            STRING_AGG(
                TO_CHAR(
                    MAKE_DATE(
                        monthly_activity.service_year,
                        monthly_activity.service_month,
                        1
                    ),
                    'FMMonth YYYY'
                ),
                ', '
                ORDER BY monthly_activity.service_year, monthly_activity.service_month
            ) FILTER (
                WHERE
                    monthly_activity.dentist_id IS NOT NULL
                    AND (
                        (
                            monthly_activity.service_year = $1
                            AND monthly_activity.service_month = $2
                        )
                        OR dentist_payments.id IS NULL
                    )
            ) AS remarks,

            MAX(monthly_activity.latest_date_service_performed) FILTER (
                WHERE
                    monthly_activity.dentist_id IS NOT NULL
                    AND (
                        (
                            monthly_activity.service_year = $1
                            AND monthly_activity.service_month = $2
                        )
                        OR dentist_payments.id IS NULL
                    )
            ) AS date_of_latest

        FROM principal_flat_fee
        LEFT JOIN monthly_activity
            ON monthly_activity.dentist_id = principal_flat_fee.dentist_id
           AND monthly_activity.clinic_id = principal_flat_fee.clinic_id

        LEFT JOIN dentist_payments
            ON dentist_payments.dentist_id = principal_flat_fee.dentist_id
           AND dentist_payments.clinic_id = principal_flat_fee.clinic_id
           AND dentist_payments.year = monthly_activity.service_year
           AND dentist_payments.month = monthly_activity.service_month

        GROUP BY
            principal_flat_fee.dentist_clinic_id,
            principal_flat_fee.dentist_id,
            principal_flat_fee.dentist_name,
            principal_flat_fee.clinic_id,
            principal_flat_fee.clinic_name,
            principal_flat_fee.retainer_fee

        ORDER BY
            principal_flat_fee.dentist_name,
            principal_flat_fee.clinic_name
    "#;

    let source_rows = DentistRetainerPayableSourceRow::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        sql,
        [
            query.year.into(),
            (query.month as i32).into(),
            selected_next_month_start.into(),
        ],
    ))
        .all(db)
        .await?;

    let rows = source_rows
        .into_iter()
        .map(|row| DentistRetainerPayableRow {
            dentist_clinic_id: row.dentist_clinic_id,

            dentist_id: row.dentist_id,
            dentist_name: row.dentist_name,

            clinic_id: row.clinic_id,
            clinic_name: row.clinic_name,

            retainer_fee: row.retainer_fee,

            rate: row.rate,
            remarks: row.remarks.unwrap_or_default(),
            date_of_latest: row.date_of_latest,
        })
        .collect::<Vec<_>>();

    let grand_total_rate = rows.iter().map(|row| row.rate).sum();

    Ok(DentistRetainerPayablesResponse {
        year: query.year,
        month: query.month,
        month_label,
        rows,
        grand_total_rate,
    })
}

// endregion: get_dentist_retainer_payables


// region: get_dentist_retainer_payables helpers
fn add_one_month(date: NaiveDate) -> Result<NaiveDate, DentistRetainerPayablesError> {
    let year = date.year();
    let month = date.month();

    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .ok_or(DentistRetainerPayablesError::InvalidDate)
}

fn format_month_year(year: i32, month: u32) -> String {
    let month_name = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };

    format!("{month_name} {year}")
}
// endregion: get_dentist_retainer_payables helpers


// region: get_dentist_retainer_payables_handler
pub async fn get_dentist_retainer_payables_handler(
    State(state): State<AppState>,
    Query(query): Query<DentistRetainerPayablesQuery>,
) -> Result<Json<DentistRetainerPayablesResponse>, (StatusCode, String)> {
    let response = get_dentist_retainer_payables(&state.db, query)
        .await
        .map_err(map_dentist_retainer_payables_error)?;

    Ok(Json(response))
}
fn map_dentist_retainer_payables_error(
    error: DentistRetainerPayablesError,
) -> (StatusCode, String) {
    match error {
        DentistRetainerPayablesError::InvalidMonth
        | DentistRetainerPayablesError::InvalidDate => {
            (StatusCode::BAD_REQUEST, error.to_string())
        }

        DentistRetainerPayablesError::Database(_) => {
            tracing::error!(?error, "Failed to get dentist retainer payables");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get dentist retainer payables".to_string(),
            )
        }
    }
}
// endregion: get_dentist_retainer_payables_handler
