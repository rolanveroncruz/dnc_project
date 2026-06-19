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
use crate::handlers::api::billing_payments::dentist_matrices::structs::{DentistHmoAuditCell, DentistHmoAuditDentistRow, DentistHmoAuditHmoColumn, DentistHmoAuditHmoTotal, DentistHmoAuditServiceLine, DentistHmoServiceAuditMatrixResponse};

// region: get_dentist_hmo_service_audit_hmo_rows
#[derive(Debug, FromQueryResult)]
pub struct DentistHmoServiceAuditHmoSourceRow {
    pub hmo_id: i32,
    pub hmo_short_name: String,
    pub hmo_long_name: String,
}

pub async fn get_dentist_hmo_service_audit_hmo_rows(
    db: &DatabaseConnection,
) -> Result<Vec<DentistHmoServiceAuditHmoSourceRow>, DbErr> {
    let sql = r#"
        SELECT
            h.id AS hmo_id,
            h.short_name AS hmo_short_name,
            h.long_name AS hmo_long_name
        FROM hmo h
        ORDER BY
            h.short_name ASC,
            h.id ASC"#;

    DentistHmoServiceAuditHmoSourceRow::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            vec![],
        )
    )
        .all(db)
        .await
}

// endregion: get_dentist_hmo_service_audit_hmo_rows

// region: get_dentist_hmo_service_audit_dentist_rows
#[derive(Debug, FromQueryResult)]
pub struct DentistHmoServiceAuditDentistSourceRow {
    pub dentist_id: i32,
    pub dentist_name: String,
    pub dentist_contract_id: i32,
    pub dentist_contract_name: String,
}

pub async fn get_dentist_hmo_service_audit_dentist_rows(
    db: &DatabaseConnection,
) -> Result<Vec<DentistHmoServiceAuditDentistSourceRow>, DbErr> {
    let sql = r#"
        SELECT
            d.id AS dentist_id,
            CONCAT_WS(
                ' ',
                CONCAT(d.last_name, ','),
                d.given_name,
                d.middle_name
            ) AS dentist_name,
            d.accre_dentist_contract_id AS dentist_contract_id,
            dc.name AS dentist_contract_name
        FROM dentist d
        INNER JOIN dentist_contract dc
            ON dc.id = d.accre_dentist_contract_id

        WHERE d.accre_dentist_contract_id IS NOT NULL

        ORDER BY
            d.last_name ASC,
            d.given_name ASC,
            d.middle_name ASC NULLS LAST,
            d.id ASC"#;

    DentistHmoServiceAuditDentistSourceRow::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            vec![],
        )
    )
        .all(db)
        .await
}

// endregion: get_dentist_hmo_service_audit_dentist_rows

// region: get_dentist_hmo_service_audit_source_rows()
#[derive(Debug, FromQueryResult)]
pub struct DentistHmoServiceAuditSourceRow {
    pub verification_id: i32,
    pub date_service_performed: Date,

    pub dentist_id: i32,
    pub dentist_name: String,
    pub dentist_contract_id: i32,
    pub dentist_contract_name: String,

    pub hmo_id: i32,
    pub hmo_short_name: String,
    pub hmo_long_name: String,

    pub dental_service_id: i32,
    pub dental_service_type_id: i32,
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
            d.accre_dentist_contract_id AS dentist_contract_id,
            dc.name AS dentist_contract_name,

            h.id AS hmo_id,
            h.short_name AS hmo_short_name,
            h.long_name AS hmo_long_name,

            ds.id AS dental_service_id,
            ds.name AS dental_service_name,
            ds.type_id AS dental_service_type_id,

            COALESCE(dcsr.rate, 0)::double precision AS service_fee

        FROM verification v

        INNER JOIN dentist d
            ON d.id = v.dentist_id

        INNER JOIN dentist_contract dc
            ON dc.id = d.accre_dentist_contract_id

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

fn format_period(start:Option<Date>, end:Option<Date>) -> String {
    match (start, end) {
        (Some(start), Some(end)) => {
            let start_label = start.format("%m/%d/%y").to_string();
            let end_label = end.format("%m/%d/%y").to_string();

            if start == end {
                start_label
            } else {
                format!("{start_label} - {end_label}")
            }
        }
        _ => "-".to_string(),
    }
}

fn update_dentist_period(
    dentist_acc: &mut DentistAccumulator,
    date_service_performed: Date
){
    dentist_acc.period_start = Some( match dentist_acc.period_start {
        Some(current_start)=>current_start.min(date_service_performed),
        None => date_service_performed,
    });
    dentist_acc.period_end = Some( match dentist_acc.period_end {
        Some(current_end)=>current_end.max(date_service_performed),
        None => date_service_performed,
    });
}
#[derive(Debug)]
struct DentistAccumulator {
    dentist_id: i32,
    dentist_name: String,
    dentist_contract_id: i32,
    dentist_contract_name: String,
    period_start: Option<Date>,
    period_end: Option<Date>,
    cells: BTreeMap<i32, CellAccumulator>,
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
    dental_service_type_id: i32,
    qty: i64,
    service_fee: f64,
}

#[derive(Debug)]
struct HmoTotalAccumulator {
    hmo_id: i32,
    hmo_short_name: String,
    hmo_long_name: String,
    total_qty: i64,
    total_fee: f64,
}

pub fn map_report_error(err: DbErr) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to generate dentist HMO service audit matrix: {err}"),
    )
}

pub fn build_dentist_hmo_service_audit_matrix(
    start_date: Date,
    end_date: Date,
    dentist_rows: Vec<DentistHmoServiceAuditDentistSourceRow>,
    hmo_rows: Vec<DentistHmoServiceAuditHmoSourceRow>,
    source_rows: Vec<DentistHmoServiceAuditSourceRow>,
) -> DentistHmoServiceAuditMatrixResponse {

    // 1 - Instantiate the accumulator maps.
    let mut hmo_map: BTreeMap<i32, DentistHmoAuditHmoColumn> = BTreeMap::new();
    let mut dentist_map: BTreeMap<i32, DentistAccumulator> = BTreeMap::new();
    let mut hmo_total_map: BTreeMap<i32, HmoTotalAccumulator> = BTreeMap::new();


    // 1a - Seed hmo_map and hmo_total_map with the hmo_rows.
    for hmo_row in hmo_rows {
        hmo_map.insert(
            hmo_row.hmo_id,
            DentistHmoAuditHmoColumn {
                hmo_id: hmo_row.hmo_id,
                hmo_short_name: hmo_row.hmo_short_name.clone(),
                hmo_long_name: hmo_row.hmo_long_name.clone(),
            },
        );

        hmo_total_map.insert(
            hmo_row.hmo_id,
            HmoTotalAccumulator{
                hmo_id: hmo_row.hmo_id,
                hmo_short_name: hmo_row.hmo_short_name.clone(),
                hmo_long_name: hmo_row.hmo_long_name.clone(),
                total_qty: 0,
                total_fee: 0.0,
            },
        );
    }
    // 1b - Seed dentist_map with the dentist_rows.
    for dentist_row in dentist_rows {
        dentist_map.insert(
            dentist_row.dentist_id,
            DentistAccumulator {
                dentist_id: dentist_row.dentist_id,
                dentist_name: dentist_row.dentist_name,
                dentist_contract_id: dentist_row.dentist_contract_id,
                dentist_contract_name: dentist_row.dentist_contract_name,
                period_start: None,
                period_end: None,
                cells: BTreeMap::new(),
            }
        );
    }


    // 2- Iterate through the source rows and populate the accumulator maps.
    // Each source row represents a verification, a service done by the dentist. Ultimately, it should be
    // added to the services field of the cell accumulator.
    for row in source_rows {

        // 2a. insert the hmo into the hmo_map if it doesn't exist.
        hmo_map
            .entry(row.hmo_id)
            .or_insert_with(|| DentistHmoAuditHmoColumn {
                hmo_id: row.hmo_id,
                hmo_short_name: row.hmo_short_name.clone(),
                hmo_long_name: row.hmo_long_name.clone(),
            });

        // 2b. insert the hmo into the hmo_total_map if it doesn't exist.
        hmo_total_map
            .entry(row.hmo_id)
            .or_insert_with(|| HmoTotalAccumulator{
                hmo_id: row.hmo_id,
                hmo_short_name: row.hmo_short_name.clone(),
                hmo_long_name: row.hmo_long_name.clone(),
                total_qty: 0,
                total_fee: 0.0,
            });

        // 2c. insert the dentist into the dentist_map if it doesn't exist;
        // also, get a reference to that dentist_accumulator instance.
        let dentist_acc = dentist_map
            .entry(row.dentist_id)
            .or_insert_with(|| DentistAccumulator {
                dentist_id: row.dentist_id,
                dentist_name: row.dentist_name.clone(),
                dentist_contract_id: row.dentist_contract_id,
                dentist_contract_name: row.dentist_contract_name.clone(),
                period_start: None,
                period_end: None,
                cells: BTreeMap::new(),
            });

        // 2c-1. adjust the dentist_accumulator's period_start and period_end if necessary.
        update_dentist_period(dentist_acc, row.date_service_performed);

        // 2c-2. insert the cellAccumulator into the dentist_accumulator's cells map if it doesn't exist;
        let cell_acc = dentist_acc
            .cells
            .entry(row.hmo_id)
            .or_insert_with(|| CellAccumulator {
                hmo_id: row.hmo_id,
                services: BTreeMap::new(),
            });

        // 2c-2-1. insert the serviceAccumulator into the cellAccumulator's services map if it doesn't exist;
        let service_acc = cell_acc
            .services
            .entry(row.dental_service_id)
            .or_insert_with(|| ServiceAccumulator {
                dental_service_id: row.dental_service_id,
                dental_service_name: row.dental_service_name.clone(),
                dental_service_type_id: row.dental_service_type_id,
                qty: 0,
                service_fee: row.service_fee,
            });

        service_acc.qty += 1;
    }
    //
    // At this point, we have completed iterating through the source rows
    // and have maps for dentists, hmos, and hmo_totals.
    //


    // Convert hmo_map to hmos, a vector.
    let mut hmos: Vec<DentistHmoAuditHmoColumn> = hmo_map.into_values().collect();

    // sort the vector according to hmo_short_name
    hmos.sort_by(|a, b| {
        a.hmo_short_name
            .to_lowercase()
            .cmp(&b.hmo_short_name.to_lowercase())
            .then(a.hmo_id.cmp(&b.hmo_id))
    });

    let mut rows: Vec<DentistHmoAuditDentistRow> = Vec::new();

    // Declare variables to hold overall grand totals
    let mut grand_total_qty: i64 = 0;
    let mut grand_total_fee: f64 = 0.0;
    let mut grand_total_basic_fee: f64 = 0.0;
    let mut grand_total_nonbasic_fee: f64 = 0.0;

    // Iterate through the dentist_map, one DentistAccumulator at a time.
    for dentist_acc in dentist_map.into_values() {
        let mut cells: Vec<DentistHmoAuditCell> = Vec::new();

        // For each dentist, we want to know totals across all cells.
        let mut row_total_qty: i64 = 0;
        let mut row_total_fee: f64 = 0.0;
        let mut total_basic_fee: f64 = 0.0;
        let mut total_nonbasic_fee: f64 = 0.0;

        // Iterate through the dentist's cells.
        for cell_acc in dentist_acc.cells.into_values() {
            let mut services: Vec<DentistHmoAuditServiceLine> = Vec::new();

            let mut cell_total_qty: i64 = 0;
            let mut cell_total_fee: f64 = 0.0;

            // Iterate through the cell's services.
            for service_acc in cell_acc.services.into_values() {

                // Compute the total fee for this service as qty * service_fee.
                let total_fee = service_acc.qty as f64 * service_acc.service_fee;

                // Add the total fee to the cell's total_fee.
                cell_total_qty += service_acc.qty;
                cell_total_fee += total_fee;

                if service_acc.dental_service_type_id == 1 {
                    total_basic_fee += total_fee;
                } else {
                    total_nonbasic_fee += total_fee;
                }

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

            if let Some(hmo_total_acc) = hmo_total_map.get_mut(&cell_acc.hmo_id) {
                hmo_total_acc.total_qty += cell_total_qty;
                hmo_total_acc.total_fee += cell_total_fee;
            }

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
        grand_total_basic_fee += total_basic_fee;
        grand_total_nonbasic_fee += total_nonbasic_fee;

        let period = format_period(dentist_acc.period_start, dentist_acc.period_end);
        let subtotal_fee = total_basic_fee + total_nonbasic_fee;
        rows.push(DentistHmoAuditDentistRow {
            dentist_id: dentist_acc.dentist_id,
            dentist_name: dentist_acc.dentist_name,
            dentist_contract_id: dentist_acc.dentist_contract_id,
            dentist_contract_name: dentist_acc.dentist_contract_name,
            period,
            cells,
            row_total_qty,
            row_total_fee,
            total_basic_fee,
            total_nonbasic_fee,
            subtotal_fee,
        });
    }

    rows.sort_by(|a, b| {
        a.dentist_contract_name
            .to_lowercase()
            .cmp(&b.dentist_contract_name.to_lowercase())
            .then(
                a.dentist_name
                    .to_lowercase()
                    .cmp(&b.dentist_name.to_lowercase())
                    .then(a.dentist_id.cmp(&b.dentist_id)),
            )
    });
    let mut hmo_totals: Vec<DentistHmoAuditHmoTotal> = hmo_total_map
        .into_values()
        .map(|acc| DentistHmoAuditHmoTotal {
            hmo_id: acc.hmo_id,
            hmo_short_name: acc.hmo_short_name,
            hmo_long_name: acc.hmo_long_name,
            total_qty: acc.total_qty,
            total_fee: acc.total_fee,
        })
        .collect();

    hmo_totals.sort_by(|a, b| {
        a.hmo_short_name
            .to_lowercase()
            .cmp(&b.hmo_short_name.to_lowercase())
            .then(a.hmo_id.cmp(&b.hmo_id))
    });


    DentistHmoServiceAuditMatrixResponse {
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        hmos,
        hmo_totals,
        rows,
        grand_total_qty,
        grand_total_fee,
        grand_total_basic_fee,
        grand_total_nonbasic_fee,
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
    let dentist_rows = get_dentist_hmo_service_audit_dentist_rows(db).await?;
    let hmo_rows = get_dentist_hmo_service_audit_hmo_rows(db).await?;
    let source_rows = get_dentist_hmo_service_audit_source_rows(db, start_date, end_date).await?;
    Ok(build_dentist_hmo_service_audit_matrix(
        start_date,
        end_date,
        dentist_rows,
        hmo_rows,
        source_rows,
    ))
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
