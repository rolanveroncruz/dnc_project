/*
These contain the functions to compute the dentist vs. hmo service audit matrix.

get_dentist_hmo_service_audit_source_rows() queries the database to get raw data arranged as Vec<DentistHmoServiceAuditSourceRow>.
1. This takes the start date, end date, and a database connection as parameters.
2. And outputs a Vec<DentistHmoServiceAuditSourceRow>.

then build_dentist_hmo_service_audit_matrix() builds the matrix from the raw data.
1. This takes the start date, end date, and the raw data (Vec<DentistHmoServiceAuditSourceRow>) as parameters;
2. And outputs a DentistHmoServiceAuditMatrixResponse.

then get_dentist_hmo_service_audit_matrix() combines the two functions to return the final matrix.

Finally, get_dentist_hmo_service_audit_matrix_handler() is the handler for the API endpoint.
1. This takes the start date, end date, and a database connection as parameters;
2. And outputs a Json<DentistHmoServiceAuditMatrixResponse>.
 */

use sea_orm::{
     DatabaseBackend, DatabaseConnection, DbErr,
     FromQueryResult, Statement,
};
use sea_orm::entity::prelude::Date;
use std::collections::BTreeMap;
use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;
use serde::Deserialize;
use tracing::instrument;
use crate::AppState;
use crate::handlers::api::billing_payments::dentist_matrices::structs::{DentistHmoAuditCell, DentistHmoAuditDentistRow, DentistHmoAuditHmoColumn, DentistHmoAuditServiceLine, DentistHmoServiceAuditMatrixResponse};

// region: get_dentist_hmo_service_audit_source_rows()
#[derive(Debug, FromQueryResult)]
pub struct DentistHmoServiceAuditSourceRow {
    pub verification_id: i32,
    pub date_service_performed: Date,

    pub dentist_id: i32,
    pub dentist_name: String,

    pub hmo_id: i32,
    pub hmo_short_name: String,
    pub hmo_long_name: String,

    pub dental_service_id: i32,
    pub dental_service_name: String,

    pub service_fee: f64,
}

pub async fn get_dentist_hmo_service_audit_source_rows(
    db: &DatabaseConnection,
    start_date: Date,
    end_date: Date,
) -> Result<Vec<DentistHmoServiceAuditSourceRow>, DbErr> {
    let sql = r#"
        SELECT
            v.id AS verification_id,
            v.date_service_performed AS date_service_performed,

            d.id AS dentist_id,
            CONCAT_WS(
                ' ',
                CONCAT(d.last_name, ','),
                d.given_name,
                d.middle_name
            ) AS dentist_name,

            h.id AS hmo_id,
            h.short_name AS hmo_short_name,
            h.long_name AS hmo_long_name,

            ds.id AS dental_service_id,
            ds.name AS dental_service_name,

            COALESCE(dcsr.rate, 0)::double precision AS service_fee

        FROM verification v

        INNER JOIN dentist d
            ON d.id = v.dentist_id

        INNER JOIN master_list_member mlm
            ON mlm.id = v.member_id

        INNER JOIN endorsement e
            ON e.id = mlm.endorsement_id

        INNER JOIN hmo h
            ON h.id = e.hmo_id

        INNER JOIN dental_service ds
            ON ds.id = v.dental_service_id

        LEFT JOIN dentist_contract_service_rates dcsr
            ON dcsr.dentist_contract_id = d.accre_dentist_contract_id
           AND dcsr.service_id = v.dental_service_id

        WHERE v.is_reconciled IS TRUE
          AND v.date_service_performed IS NOT NULL
          AND v.date_service_performed BETWEEN $1::date AND $2::date
          AND d.accre_dentist_contract_id IS NOT NULL

        ORDER BY
            d.last_name ASC,
            d.given_name ASC,
            d.middle_name ASC NULLS LAST,
            h.short_name ASC,
            ds.name ASC,
            v.id ASC
    "#;

    DentistHmoServiceAuditSourceRow::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            vec![start_date.into(), end_date.into()],
        )
    )
        .all(db)
        .await
}


// endregion: get_dentist_hmo_service_audit_source_rows()

// region: build_dentist_hmo_service_audit_matrix()
#[derive(Debug)]
struct DentistAccumulator {
    dentist_id: i32,
    dentist_name: String,
    cells: BTreeMap<i32, CellAccumulator>,
}

fn map_report_error(err: DbErr) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to generate dentist HMO service audit matrix: {err}"),
    )
}
#[derive(Debug)]
struct CellAccumulator {
    hmo_id: i32,
    services: BTreeMap<i32, ServiceAccumulator>,
}

#[derive(Debug)]
struct ServiceAccumulator {
    dental_service_id: i32,
    dental_service_name: String,
    qty: i64,
    service_fee: f64,
}

pub fn build_dentist_hmo_service_audit_matrix(
    start_date: Date,
    end_date: Date,
    source_rows: Vec<DentistHmoServiceAuditSourceRow>,
) -> DentistHmoServiceAuditMatrixResponse {
    let mut hmo_map: BTreeMap<i32, DentistHmoAuditHmoColumn> = BTreeMap::new();
    let mut dentist_map: BTreeMap<i32, DentistAccumulator> = BTreeMap::new();

    for row in source_rows {
        hmo_map
            .entry(row.hmo_id)
            .or_insert_with(|| DentistHmoAuditHmoColumn {
                hmo_id: row.hmo_id,
                hmo_short_name: row.hmo_short_name.clone(),
                hmo_long_name: row.hmo_long_name.clone(),
            });

        let dentist_acc = dentist_map
            .entry(row.dentist_id)
            .or_insert_with(|| DentistAccumulator {
                dentist_id: row.dentist_id,
                dentist_name: row.dentist_name.clone(),
                cells: BTreeMap::new(),
            });

        let cell_acc = dentist_acc
            .cells
            .entry(row.hmo_id)
            .or_insert_with(|| CellAccumulator {
                hmo_id: row.hmo_id,
                services: BTreeMap::new(),
            });

        let service_acc = cell_acc
            .services
            .entry(row.dental_service_id)
            .or_insert_with(|| ServiceAccumulator {
                dental_service_id: row.dental_service_id,
                dental_service_name: row.dental_service_name.clone(),
                qty: 0,
                service_fee: row.service_fee,
            });

        service_acc.qty += 1;
    }

    let mut hmos: Vec<DentistHmoAuditHmoColumn> = hmo_map.into_values().collect();

    hmos.sort_by(|a, b| {
        a.hmo_short_name
            .to_lowercase()
            .cmp(&b.hmo_short_name.to_lowercase())
            .then(a.hmo_id.cmp(&b.hmo_id))
    });

    let mut rows: Vec<DentistHmoAuditDentistRow> = Vec::new();

    let mut grand_total_qty: i64 = 0;
    let mut grand_total_fee: f64 = 0.0;

    for dentist_acc in dentist_map.into_values() {
        let mut cells: Vec<DentistHmoAuditCell> = Vec::new();

        let mut row_total_qty: i64 = 0;
        let mut row_total_fee: f64 = 0.0;

        for cell_acc in dentist_acc.cells.into_values() {
            let mut services: Vec<DentistHmoAuditServiceLine> = Vec::new();

            let mut cell_total_qty: i64 = 0;
            let mut cell_total_fee: f64 = 0.0;

            for service_acc in cell_acc.services.into_values() {
                let total_fee = service_acc.qty as f64 * service_acc.service_fee;

                cell_total_qty += service_acc.qty;
                cell_total_fee += total_fee;

                services.push(DentistHmoAuditServiceLine {
                    dental_service_id: service_acc.dental_service_id,
                    dental_service_name: service_acc.dental_service_name,
                    qty: service_acc.qty,
                    service_fee: service_acc.service_fee,
                    total_fee,
                });
            }

            services.sort_by(|a, b| {
                a.dental_service_name
                    .to_lowercase()
                    .cmp(&b.dental_service_name.to_lowercase())
                    .then(a.dental_service_id.cmp(&b.dental_service_id))
            });

            row_total_qty += cell_total_qty;
            row_total_fee += cell_total_fee;

            cells.push(DentistHmoAuditCell {
                hmo_id: cell_acc.hmo_id,
                services,
                cell_total_qty,
                cell_total_fee,
            });
        }

        cells.sort_by(|a, b| a.hmo_id.cmp(&b.hmo_id));

        grand_total_qty += row_total_qty;
        grand_total_fee += row_total_fee;

        rows.push(DentistHmoAuditDentistRow {
            dentist_id: dentist_acc.dentist_id,
            dentist_name: dentist_acc.dentist_name,
            cells,
            row_total_qty,
            row_total_fee,
        });
    }

    rows.sort_by(|a, b| {
        a.dentist_name
            .to_lowercase()
            .cmp(&b.dentist_name.to_lowercase())
            .then(a.dentist_id.cmp(&b.dentist_id))
    });

    DentistHmoServiceAuditMatrixResponse {
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        hmos,
        rows,
        grand_total_qty,
        grand_total_fee,
    }
}
// endregion: build_dentist_hmo_service_audit_matrix()

// region: get_dentist_hmo_service_audit_matrix()
// get_dentist_hmo_service_audit_matrix() queries the database to get the dentist vs. hmo service audit matrix.
// 1. This takes the start date, end date, and a database connection as parameters;
// 2. And outputs a DentistHmoServiceAuditMatrixResponse.
//
pub async fn get_dentist_hmo_service_audit_matrix(
    db: &DatabaseConnection,
    start_date: Date,
    end_date: Date,
) -> Result<DentistHmoServiceAuditMatrixResponse, DbErr> {
    let source_rows = get_dentist_hmo_service_audit_source_rows(db, start_date, end_date).await?;
    Ok(build_dentist_hmo_service_audit_matrix(start_date, end_date, source_rows))
}
// endregion: get_dentist_hmo_service_audit_matrix()

// region: get_dentist_hmo_service_audit_matrix_handler()
#[derive(Debug, Deserialize)]
pub struct DentistHmoServiceAuditMatrixQuery {
    pub start_date: Date,
    pub end_date: Date,
}
#[instrument(skip(state))]
pub async fn get_dentist_hmo_service_audit_matrix_handler(
    State(state): State<AppState>,
    Query(query): Query<DentistHmoServiceAuditMatrixQuery>,
) -> Result<Json<DentistHmoServiceAuditMatrixResponse>, (StatusCode, String)> {
    if query.start_date > query.end_date {
        return Err((
            StatusCode::BAD_REQUEST,
            "start_date must be earlier than or equal to end_date".to_string(),
        ));
    }

    let report = get_dentist_hmo_service_audit_matrix(
        &state.db,
        query.start_date,
        query.end_date,
    )
        .await
        .map_err(map_report_error)?;

    Ok(Json(report))
}
// endregion: get_dentist_hmo_service_audit_matrix_handler()
