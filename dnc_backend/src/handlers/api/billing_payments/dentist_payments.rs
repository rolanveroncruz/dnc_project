use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use axum::extract::{Path, State};
use axum::Json;
use chrono::{Datelike, FixedOffset, Utc};
use http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
use tracing::instrument;
use crate::AppState;
use crate::entities::{
    dental_clinic, dentist, dentist_clinic, dentist_payments,
};
#[derive(Debug, Clone, Serialize)]
pub struct DentistPaymentMatrixResponse {
    pub months: Vec<DentistPaymentMatrixMonth>,
    pub rows: Vec<DentistPaymentMatrixRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistPaymentMatrixMonth {
    pub year: i32,
    pub month: i32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistPaymentMatrixRow {
    pub dentist_clinic_id: i32,
    pub dentist_id: i32,
    pub dentist_name: String,
    pub clinic_id: i32,
    pub clinic_name: String,
    pub cells: Vec<DentistPaymentMatrixCell>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistPaymentMatrixCell {
    pub year: i32,
    pub month: i32,
    pub paid: bool,
    pub payment_id: Option<i32>,
    pub report_name: Option<String>,
    pub date_paid: Option<DateTimeWithTimeZone>,
    pub date_paid_recorded_by: Option<String>,
}

#[derive(Debug, FromQueryResult)]
struct PrincipalDentistClinicQueryRow {
    dentist_clinic_id: i32,
    dentist_id: i32,
    dentist_last_name: String,
    dentist_given_name: String,
    dentist_middle_name: Option<String>,
    clinic_id: i32,
    clinic_name: String,
}
// region: get_dentist_payment_matrix

fn format_dentist_name(
    last_name: &str,
    given_name: &str,
    middle_name: Option<&str>,
) -> String {
    match middle_name {
        Some(middle) if !middle.trim().is_empty() => {
            format!("{}, {} {}", last_name, given_name, middle)
        }
        _ => format!("{}, {}", last_name, given_name),
    }
}

fn build_last_12_months_ending_current_month() -> Vec<DentistPaymentMatrixMonth> {
    // Use Manila time so "current month" follows the business timezone.
    let manila_offset = FixedOffset::east_opt(8 * 60 * 60)
        .expect("valid Manila UTC offset");

    let now = Utc::now().with_timezone(&manila_offset);

    let current_year = now.year();
    let current_month = now.month() as i32;

    let mut months = Vec::with_capacity(12);

    for offset in (0..12).rev() {
        let mut year = current_year;
        let mut month = current_month - offset;

        while month <= 0 {
            month += 12;
            year -= 1;
        }

        let label = format_month_label(year, month);

        months.push(DentistPaymentMatrixMonth {
            year,
            month,
            label,
        });
    }

    months
}

fn format_month_label(year: i32, month: i32) -> String {
    let month_name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };

    format!("{} {}", month_name, year)
}

pub async fn get_dentist_payment_matrix(
    db: &DatabaseConnection,
) -> Result<DentistPaymentMatrixResponse, DbErr> {
    let months = build_last_12_months_ending_current_month();

    let month_pairs: HashSet<(i32, i32)> = months
        .iter()
        .map(|month| (month.year, month.month))
        .collect();

    let first_month = months
        .first()
        .expect("last 12 months should not be empty");

    let last_month = months
        .last()
        .expect("last 12 months should not be empty");

    // Build the rows of the matrix. Start with the dentist_clinic, join it with dentist and dental_clinic.
    // but only get the principal dentists with flat free contracts.
    let principal_rows: Vec<PrincipalDentistClinicQueryRow> = dentist_clinic::Entity::find()
        .join(
            JoinType::InnerJoin,
            dentist_clinic::Relation::Dentist.def(),
        )
        .join(
            JoinType::InnerJoin,
            dentist_clinic::Relation::DentalClinic.def(),
        )
        .filter(dentist_clinic::Column::PositionId.eq(1))
        .filter(dentist_clinic::Column::ClinicId.is_not_null())
        .filter(dentist::Column::AccreDentistContractId.eq(1))
        .select_only()
        // return the columns
        .column_as(dentist_clinic::Column::Id, "dentist_clinic_id")
        .column_as(dentist_clinic::Column::DentistId, "dentist_id")
        .column_as(dentist_clinic::Column::ClinicId, "clinic_id")
        .column_as(dentist::Column::LastName, "dentist_last_name")
        .column_as(dentist::Column::GivenName, "dentist_given_name")
        .column_as(dentist::Column::MiddleName, "dentist_middle_name")
        .column_as(dental_clinic::Column::Name, "clinic_name")
        .order_by_asc(dentist::Column::LastName)
        .order_by_asc(dentist::Column::GivenName)
        .order_by_asc(dental_clinic::Column::Name)
        .into_model::<PrincipalDentistClinicQueryRow>()
        .all(db)
        .await?;

    let dentist_ids: HashSet<i32> = principal_rows
        .iter()
        .map(|row| row.dentist_id)
        .collect();

    let clinic_ids: HashSet<i32> = principal_rows
        .iter()
        .map(|row| row.clinic_id)
        .collect();

    let payments = if dentist_ids.is_empty() || clinic_ids.is_empty() {
        Vec::new()
    } else {
        dentist_payments::Entity::find()
            .filter(dentist_payments::Column::DentistId.is_in(dentist_ids))
            .filter(dentist_payments::Column::ClinicId.is_in(clinic_ids))
            .filter(
                dentist_payments::Column::Year
                    .gte(first_month.year)
            )
            .filter(dentist_payments::Column::Year.lte(last_month.year))
            .all(db)
            .await?
    };

    let mut payment_lookup = HashMap::new();

    for payment in payments {
        let key = (
            payment.dentist_id,
            payment.clinic_id,
            payment.year,
            payment.month,
        );

        // Keep only payments inside the exact 12-month window.
        // This matters when the 12-month window crosses years.
        if month_pairs.contains(&(payment.year, payment.month)) {
            payment_lookup.insert(key, payment);
        }
    }

    let rows = principal_rows
        .into_iter()
        .map(|row| {
            let dentist_name = format_dentist_name(
                &row.dentist_last_name,
                &row.dentist_given_name,
                row.dentist_middle_name.as_deref(),
            );

            let cells = months
                .iter()
                .map(|month| {
                    let key = (
                        row.dentist_id,
                        row.clinic_id,
                        month.year,
                        month.month,
                    );

                    let payment = payment_lookup.get(&key);

                    DentistPaymentMatrixCell {
                        year: month.year,
                        month: month.month,
                        paid: payment.is_some(),
                        payment_id: payment.map(|payment| payment.id),
                        report_name: payment.and_then(|payment| payment.report_name.clone()),
                        date_paid: payment.and_then(|payment| payment.date_paid),
                        date_paid_recorded_by: payment
                            .and_then(|payment| payment.date_paid_recorded_by.clone()),
                    }
                })
                .collect();

            DentistPaymentMatrixRow {
                dentist_clinic_id: row.dentist_clinic_id,
                dentist_id: row.dentist_id,
                dentist_name,
                clinic_id: row.clinic_id,
                clinic_name: row.clinic_name,
                cells,
            }
        })
        .collect();

    Ok(DentistPaymentMatrixResponse { months, rows })
}

#[instrument(skip(state))]
pub async fn get_dentist_payment_matrix_handler(
    State(state): State<AppState>,
) -> Result<Json<DentistPaymentMatrixResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let response = get_dentist_payment_matrix(&state.db)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to generate dentist payment matrix");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    error: "failed to generate dentist payment matrix".to_string(),
                }),
            )
        })?;

    Ok(Json(response))
}

// endregion: get_dentist_payment_matrix


// region payment structs:
#[derive(Debug, Clone, Deserialize)]
pub struct MakeDentistPaymentRequest {
    pub dentist_id: i32,
    pub clinic_id: i32,
    pub year: i32,
    pub month: i32,
    pub report_name: Option<String>,
    pub date_paid_recorded_by: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DentistPaymentResponse {
    pub id: i32,
    pub dentist_id: i32,
    pub clinic_id: i32,
    pub year: i32,
    pub month: i32,
    pub report_name: Option<String>,
    pub date_paid: Option<DateTimeWithTimeZone>,
    pub date_paid_recorded_by: Option<String>,
    pub created: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteDentistPaymentResponse {
    pub deleted: bool,
    pub payment_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}
// endregion payment structs:

// region: make dentist_payment
pub async fn make_dentist_payment(
    db: &DatabaseConnection,
    request: MakeDentistPaymentRequest,
) -> Result<DentistPaymentResponse, DbErr> {
    if request.month < 1 || request.month > 12 {
        return Err(DbErr::Custom("month must be between 1 and 12".to_string()));
    }

    // ✅ Validate that this dentist-clinic combination is eligible for payment.
    // Principal only, flat-fee contract only.
    let eligible_dentist_clinic = dentist_clinic::Entity::find()
        .join(
            JoinType::InnerJoin,
            dentist_clinic::Relation::Dentist.def(),
        )
        .filter(dentist_clinic::Column::DentistId.eq(request.dentist_id))
        .filter(dentist_clinic::Column::ClinicId.eq(request.clinic_id))
        .filter(dentist_clinic::Column::PositionId.eq(1))
        .filter(dentist::Column::AccreDentistContractId.eq(1))
        .one(db)
        .await?;

    if eligible_dentist_clinic.is_none() {
        return Err(DbErr::Custom(
            "payment is allowed only for principal dentists with flat-fee contract for the selected clinic"
                .to_string(),
        ));
    }

    // ✅ Idempotency: if already paid, return the existing payment.
    let existing_payment = dentist_payments::Entity::find()
        .filter(dentist_payments::Column::DentistId.eq(request.dentist_id))
        .filter(dentist_payments::Column::ClinicId.eq(request.clinic_id))
        .filter(dentist_payments::Column::Year.eq(request.year))
        .filter(dentist_payments::Column::Month.eq(request.month))
        .one(db)
        .await?;

    if let Some(payment) = existing_payment {
        return Ok(DentistPaymentResponse {
            id: payment.id,
            dentist_id: payment.dentist_id,
            clinic_id: payment.clinic_id,
            year: payment.year,
            month: payment.month,
            report_name: payment.report_name,
            date_paid: payment.date_paid,
            date_paid_recorded_by: payment.date_paid_recorded_by,
            created: false,
        });
    }

    let manila_offset = FixedOffset::east_opt(8 * 60 * 60)
        .expect("valid Manila UTC offset");

    let now_manila = Utc::now().with_timezone(&manila_offset);

    let active_model = dentist_payments::ActiveModel {
        dentist_id: Set(request.dentist_id),
        clinic_id: Set(request.clinic_id),
        year: Set(request.year),
        month: Set(request.month),
        report_name: Set(request.report_name),
        date_paid: Set(Some(now_manila)),
        date_paid_recorded_by: Set(request.date_paid_recorded_by),
        ..Default::default()
    };

    let inserted = active_model.insert(db).await?;

    Ok(DentistPaymentResponse {
        id: inserted.id,
        dentist_id: inserted.dentist_id,
        clinic_id: inserted.clinic_id,
        year: inserted.year,
        month: inserted.month,
        report_name: inserted.report_name,
        date_paid: inserted.date_paid,
        date_paid_recorded_by: inserted.date_paid_recorded_by,
        created: true,
    })
}
#[instrument(skip(state))]
pub async fn make_dentist_payment_handler(
    State(state): State<AppState>,
    Json(request): Json<MakeDentistPaymentRequest>,
) -> Result<Json<DentistPaymentResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let response = make_dentist_payment(&state.db, request)
        .await
        .map_err(|err| {
            tracing::error!(?err, "failed to make dentist payment");

            let message = err.to_string();

            if message.contains("month must be between 1 and 12")
                || message.contains("payment is allowed only")
            {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiErrorResponse { error: message }),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse {
                        error: "failed to make dentist payment".to_string(),
                    }),
                )
            }
        })?;

    Ok(Json(response))
}

// endregion: make dentist_payment


// region: delete payment
pub async fn delete_dentist_payment(
    db: &DatabaseConnection,
    payment_id: i32,
) -> Result<DeleteDentistPaymentResponse, DbErr> {
    let delete_result = dentist_payments::Entity::delete_by_id(payment_id)
        .exec(db)
        .await?;

    Ok(DeleteDentistPaymentResponse {
        deleted: delete_result.rows_affected > 0,
        payment_id,
    })
}


#[instrument(skip(state))]
pub async fn delete_dentist_payment_handler(
    State(state): State<AppState>,
    Path(payment_id): Path<i32>,
) -> Result<Json<DeleteDentistPaymentResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let response = delete_dentist_payment(&state.db, payment_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, payment_id, "failed to delete dentist payment");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    error: "failed to delete dentist payment".to_string(),
                }),
            )
        })?;

    Ok(Json(response))
}
// endregion: delete payment
