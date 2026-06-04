use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{
    DatabaseBackend, FromQueryResult, Statement,
};
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

use crate::AppState;

// region: Get Dentist Clinics Reconciled Jobs Count Last 12 Months

#[derive(Debug, Serialize, FromQueryResult)]
pub struct DentistClinicReconciledJobsCountLast12MonthsRow {
    pub id: Option<i32>,
    pub dentist_name: Option<String>,
    pub clinic_name: Option<String>,
    pub position_name: Option<String>,
    pub contract_name: Option<String>,
    pub month_start: Option<chrono::NaiveDateTime>,
    pub month_label: Option<String>,
    pub rec_services_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupKey {
    id: Option<i32>,
    dentist_name: Option<String>,
    clinic_name: Option<String>,
    position_name: Option<String>,
    contract_name: Option<String>,
}

pub async fn get_dentist_clinics_reconciled_jobs_count_last_12_months(
    State(state): State<AppState>,
) -> Result<Json<Vec<Value>>, (StatusCode, String)> {
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
                format!("Database error: {:?}", err),
            )
        })?;

    let mut grouped: HashMap<GroupKey, Map<String, Value>> = HashMap::new();

    for row in rows {
        let key = GroupKey {
            id: row.id,
            dentist_name: row.dentist_name.clone(),
            clinic_name: row.clinic_name.clone(),
            position_name: row.position_name.clone(),
            contract_name: row.contract_name.clone(),
        };

        let entry = grouped.entry(key.clone()).or_insert_with(|| {
            let mut map = Map::new();

            map.insert("id".to_string(), json!(key.id));
            map.insert("dentist_name".to_string(), json!(key.dentist_name));
            map.insert("clinic_name".to_string(), json!(key.clinic_name));
            map.insert("position_name".to_string(), json!(key.position_name));
            map.insert("contract_name".to_string(), json!(key.contract_name));

            map
        });

        if let Some(month_label) = row.month_label {
            entry.insert(
                month_label,
                json!(row.rec_services_count.unwrap_or(0)),
            );
        }
    }

    let result: Vec<Value> = grouped
        .into_values()
        .map(Value::Object)
        .collect();

    Ok(Json(result))
}

// endregion