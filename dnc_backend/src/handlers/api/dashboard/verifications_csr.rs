use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Datelike, Days, FixedOffset, NaiveDate, TimeZone};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, QuerySelect, Statement
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    entities::{role, user, verification},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CsrVerificationActivityQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub dental_service_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CsrVerificationActivityRow {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub created_count: i64,
    pub approved_count: i64,
    pub reconciled_count: i64,
}

#[derive(Debug, FromQueryResult)]
struct CountByEmail {
    email: String,
    count: i64,
}


// region: get_csr_verification_activity_counts
// get_csr_verification_activity_counts returns for each and only the CSR user the number of
// verifications created, approved, and reconciled in a given date interval.
pub async fn get_csr_verification_activity_counts(
    State(state): State<AppState>,
    Query(params): Query<CsrVerificationActivityQuery>,
) -> Result<Json<Vec<CsrVerificationActivityRow>>, (StatusCode, String)> {
    if params.start_date > params.end_date {
        return Err((
            StatusCode::BAD_REQUEST,
            "start_date must be before or equal to end_date".to_string(),
        ));
    }

    // Treat date interval as Manila calendar days:
    // start_date 00:00:00 up to end_date + 1 day 00:00:00
    let manila_offset = FixedOffset::east_opt(8 * 60 * 60)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create Manila timezone offset".to_string(),
            )
        })?;

    let start_naive = params
        .start_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid start_date".to_string(),
            )
        })?;

    let end_plus_one = params
        .end_date
        .checked_add_days(Days::new(1))
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end_date".to_string(),
            )
        })?;

    let end_naive = end_plus_one
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end_date".to_string(),
            )
        })?;

    let start_dt = manila_offset
        .from_local_datetime(&start_naive)
        .single()
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid start datetime".to_string(),
            )
        })?;

    let end_dt = manila_offset
        .from_local_datetime(&end_naive)
        .single()
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end datetime".to_string(),
            )
        })?;

    // 1. Get active CSR users
    let csr_users = user::Entity::find()
        .find_also_related(role::Entity)
        .filter(user::Column::Active.eq(true))
        .filter(role::Column::Name.eq("CSR"))
        .order_by_asc(user::Column::Name)
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    let mut rows: Vec<CsrVerificationActivityRow> = csr_users
        .into_iter()
        .filter_map(|(u, maybe_role)| {
            // Defensive check. The role filter should already enforce this.
            if maybe_role.map(|r| r.name) != Some("CSR".to_string()) {
                return None;
            }

            Some(CsrVerificationActivityRow {
                user_id: u.id,
                name: u.name,
                email: u.email,
                created_count: 0,
                approved_count: 0,
                reconciled_count: 0,
            })
        })
        .collect();

    // 2. Count created_by
    let mut created_query = verification::Entity::find()
        .select_only()
        .column_as(verification::Column::CreatedBy, "email")
        .column_as(verification::Column::Id.count(), "count")
        .filter(verification::Column::DateCreated.gte(start_dt))
        .filter(verification::Column::DateCreated.lt(end_dt))
        .group_by(verification::Column::CreatedBy);

    if let Some(dental_service_id) = params.dental_service_id {
        created_query = created_query
            .filter(verification::Column::DentalServiceId.eq(dental_service_id));
    }

    let created_counts = created_query
        .into_model::<CountByEmail>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    // 3. Count approved_by
    let mut approved_query = verification::Entity::find()
        .select_only()
        .column_as(verification::Column::ApprovedBy, "email")
        .column_as(verification::Column::Id.count(), "count")
        .filter(verification::Column::ApprovedBy.is_not_null())
        .filter(verification::Column::ApprovalDate.gte(start_dt))
        .filter(verification::Column::ApprovalDate.lt(end_dt))
        .group_by(verification::Column::ApprovedBy);

    if let Some(dental_service_id) = params.dental_service_id {
        approved_query = approved_query
            .filter(verification::Column::DentalServiceId.eq(dental_service_id));
    }

    let approved_counts = approved_query
        .into_model::<CountByEmail>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    // 4. Count reconciled_by
    let mut reconciled_query = verification::Entity::find()
        .select_only()
        .column_as(verification::Column::ReconciledBy, "email")
        .column_as(verification::Column::Id.count(), "count")
        .filter(verification::Column::ReconciledBy.is_not_null())
        .filter(verification::Column::ReconciliationDate.gte(start_dt))
        .filter(verification::Column::ReconciliationDate.lt(end_dt))
        .group_by(verification::Column::ReconciledBy);

    if let Some(dental_service_id) = params.dental_service_id {
        reconciled_query = reconciled_query
            .filter(verification::Column::DentalServiceId.eq(dental_service_id));
    }

    let reconciled_counts = reconciled_query
        .into_model::<CountByEmail>()
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    // 5. Merge counts into CSR users by email
    let created_map: HashMap<String, i64> = created_counts
        .into_iter()
        .map(|row| (row.email, row.count))
        .collect();

    let approved_map: HashMap<String, i64> = approved_counts
        .into_iter()
        .map(|row| (row.email, row.count))
        .collect();

    let reconciled_map: HashMap<String, i64> = reconciled_counts
        .into_iter()
        .map(|row| (row.email, row.count))
        .collect();

    for row in rows.iter_mut() {
        row.created_count = *created_map.get(&row.email).unwrap_or(&0);
        row.approved_count = *approved_map.get(&row.email).unwrap_or(&0);
        row.reconciled_count = *reconciled_map.get(&row.email).unwrap_or(&0);
    }

    Ok(Json(rows))
}


// endregion: get_csr_verification_activity_counts

// region: helper functions
fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}


// endregion: helper functions


// region: get_csr_verification_activity_unit_counts
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CsrVerificationActivityUnit {
    Day,
    Week,
    Month,
}

impl CsrVerificationActivityUnit {
    fn sql_date_trunc_unit(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
        }
    }

    fn first_bucket_start(self, date: NaiveDate) -> Result<NaiveDate, (StatusCode, String)> {
        match self {
            Self::Day => Ok(date),

            // PostgreSQL date_trunc('week', ...) starts weeks on Monday
            Self::Week => {
                let days_from_monday = date.weekday().num_days_from_monday() as u64;

                date.checked_sub_days(Days::new(days_from_monday))
                    .ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            "Invalid week bucket start".to_string(),
                        )
                    })
            }

            Self::Month => {
                NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
                    .ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            "Invalid month bucket start".to_string(),
                        )
                    })
            }
        }
    }

    fn next_bucket_start(self, date: NaiveDate) -> Result<NaiveDate, (StatusCode, String)> {
        match self {
            Self::Day => {
                date.checked_add_days(Days::new(1))
                    .ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            "Invalid next day bucket".to_string(),
                        )
                    })
            }

            Self::Week => {
                date.checked_add_days(Days::new(7))
                    .ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            "Invalid next week bucket".to_string(),
                        )
                    })
            }

            Self::Month => {
                let (year, month) = if date.month() == 12 {
                    (date.year() + 1, 1)
                } else {
                    (date.year(), date.month() + 1)
                };

                NaiveDate::from_ymd_opt(year, month, 1)
                    .ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            "Invalid next month bucket".to_string(),
                        )
                    })
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CsrVerificationActivityUnitQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub unit: CsrVerificationActivityUnit,
    pub dental_service_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CsrVerificationActivityUnitRow {
    pub period_start: NaiveDate,
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub created_count: i64,
    pub approved_count: i64,
    pub reconciled_count: i64,
}

#[derive(Debug, FromQueryResult)]
struct CountByPeriodEmail {
    email: String,
    period_start: NaiveDate,
    count: i64,
}

// region: get_csr_verification_activity_unit_counts
// get_csr_verification_activity_unit_counts returns CSR verification activity counts
// broken down by day, week, or month.
pub async fn get_csr_verification_activity_unit_counts(
    State(state): State<AppState>,
    Query(params): Query<CsrVerificationActivityUnitQuery>,
) -> Result<Json<Vec<CsrVerificationActivityUnitRow>>, (StatusCode, String)> {
    if params.start_date > params.end_date {
        return Err((
            StatusCode::BAD_REQUEST,
            "start_date must be before or equal to end_date".to_string(),
        ));
    }

    let manila_offset = FixedOffset::east_opt(8 * 60 * 60)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create Manila timezone offset".to_string(),
            )
        })?;

    let start_naive = params
        .start_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid start_date".to_string(),
            )
        })?;

    let end_plus_one = params
        .end_date
        .checked_add_days(Days::new(1))
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end_date".to_string(),
            )
        })?;

    let end_naive = end_plus_one
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end_date".to_string(),
            )
        })?;

    let start_dt = manila_offset
        .from_local_datetime(&start_naive)
        .single()
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid start datetime".to_string(),
            )
        })?;

    let end_dt = manila_offset
        .from_local_datetime(&end_naive)
        .single()
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid end datetime".to_string(),
            )
        })?;

    // 1. Get active CSR users
    let csr_users = user::Entity::find()
        .find_also_related(role::Entity)
        .filter(user::Column::Active.eq(true))
        .filter(role::Column::Name.eq("CSR"))
        .order_by_asc(user::Column::Name)
        .all(&state.db)
        .await
        .map_err(internal_error)?;

    let csr_users: Vec<_> = csr_users
        .into_iter()
        .filter_map(|(u, maybe_role)| {
            if maybe_role.map(|r| r.name) != Some("CSR".to_string()) {
                return None;
            }

            Some(u)
        })
        .collect();

    // 2. Build all period buckets, so missing days/weeks/months still return zero rows.
    let mut periods: Vec<NaiveDate> = Vec::new();

    let mut current_period = params.unit.first_bucket_start(params.start_date)?;

    while current_period <= params.end_date {
        periods.push(current_period);
        current_period = params.unit.next_bucket_start(current_period)?;
    }

    // 3. Initialize all CSR + period combinations to zero.
    let mut rows: Vec<CsrVerificationActivityUnitRow> = Vec::new();
    let mut row_index: HashMap<(NaiveDate, String), usize> = HashMap::new();

    for period_start in periods {
        for u in csr_users.iter() {
            let index = rows.len();

            row_index.insert((period_start, u.email.clone()), index);

            rows.push(CsrVerificationActivityUnitRow {
                period_start,
                user_id: u.id,
                name: u.name.clone(),
                email: u.email.clone(),
                created_count: 0,
                approved_count: 0,
                reconciled_count: 0,
            });
        }
    }

    // 4. Query created/approved/reconciled counts grouped by period + email.
    let created_counts = count_verification_activity_by_period(
        &state.db,
        params.unit,
        "date_created",
        "created_by",
        start_dt,
        end_dt,
        params.dental_service_id,
    )
        .await
        .map_err(internal_error)?;

    let approved_counts = count_verification_activity_by_period(
        &state.db,
        params.unit,
        "approval_date",
        "approved_by",
        start_dt,
        end_dt,
        params.dental_service_id,
    )
        .await
        .map_err(internal_error)?;

    let reconciled_counts = count_verification_activity_by_period(
        &state.db,
        params.unit,
        "reconciliation_date",
        "reconciled_by",
        start_dt,
        end_dt,
        params.dental_service_id,
    )
        .await
        .map_err(internal_error)?;

    // 5. Merge counts into initialized rows.
    for count_row in created_counts {
        if let Some(index) = row_index.get(&(count_row.period_start, count_row.email)) {
            rows[*index].created_count = count_row.count;
        }
    }

    for count_row in approved_counts {
        if let Some(index) = row_index.get(&(count_row.period_start, count_row.email)) {
            rows[*index].approved_count = count_row.count;
        }
    }

    for count_row in reconciled_counts {
        if let Some(index) = row_index.get(&(count_row.period_start, count_row.email)) {
            rows[*index].reconciled_count = count_row.count;
        }
    }

    Ok(Json(rows))
}

// endregion: get_csr_verification_activity_unit_counts


// region: more helper functions: count_verification_activity_by_period
async fn count_verification_activity_by_period(
    db: &DatabaseConnection,
    unit: CsrVerificationActivityUnit,
    timestamp_column: &'static str,
    user_column: &'static str,
    start_dt: DateTime<FixedOffset>,
    end_dt: DateTime<FixedOffset>,
    dental_service_id: Option<i32>,
) -> Result<Vec<CountByPeriodEmail>, DbErr> {
    let sql_unit = unit.sql_date_trunc_unit();

    let mut sql = format!(
        r#"
        SELECT
            {user_column} AS email,
            date_trunc('{sql_unit}', {timestamp_column} AT TIME ZONE 'Asia/Manila')::date AS period_start,
            COUNT(id)::bigint AS count
        FROM verification
        WHERE {user_column} IS NOT NULL
          AND {timestamp_column} >= $1
          AND {timestamp_column} < $2
        "#
    );

    let mut values = vec![start_dt.into(), end_dt.into()];

    if let Some(dental_service_id) = dental_service_id {
        sql.push_str(" AND dental_service_id = $3 ");
        values.push(dental_service_id.into());
    }

    sql.push_str(
        r#"
        GROUP BY {user_column}, period_start
        ORDER BY period_start ASC, email ASC
        "#,
    );

    // Because sql.push_str above is not format-aware, replace the literal placeholder.
    let sql = sql.replace("{user_column}", user_column);

    CountByPeriodEmail::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        values,
    ))
        .all(db)
        .await
}

// endregion: more helper functions: count_verification_activity_by_period
