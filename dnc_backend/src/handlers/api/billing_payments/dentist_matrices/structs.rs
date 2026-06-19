use serde::Serialize;


// The DentistHmoAuditHmoColumn represents and HMO (column) in the audit matrix.
#[derive(Debug, Serialize)]
pub struct DentistHmoAuditHmoColumn {
    pub hmo_id: i32,
    pub hmo_short_name: String,
    pub hmo_long_name: String,
}

#[derive(Debug, Serialize)]
pub struct DentistHmoAuditHmoTotal {
    pub hmo_id: i32,
    pub hmo_short_name: String,
    pub hmo_long_name: String,
    pub total_qty: i64,
    pub total_fee: f64,
}

// The DentistHmoAuditServiceLine represents the services performed by that dentist for the hmo.
#[derive(Debug, Serialize)]
pub struct DentistHmoAuditServiceLine {
    pub dental_service_id: i32,
    pub dental_service_name: String,

    pub qty: i64,
    pub service_fee: f64,
    pub total_fee: f64,
}

// The DentistHmoAuditCell represents an intersection between an Hmo and a dentist in the audit matrix.
// The DentistHmoAuditServiceLine represents the services performed by that dentist for the hmo.
#[derive(Debug, Serialize)]
pub struct DentistHmoAuditCell {
    pub hmo_id: i32,

    pub services: Vec<DentistHmoAuditServiceLine>,

    pub cell_total_qty: i64,
    pub cell_total_fee: f64,
}
// DentistHmoAuditDentistRow represents a row in the audit matrix.
// It has dentist information, as well as a list of AuditCells
#[derive(Debug, Serialize)]
pub struct DentistHmoAuditDentistRow {
    pub dentist_id: i32,
    pub dentist_name: String,
    pub dentist_contract_id: i32,
    pub dentist_contract_name: String,
    pub period: String,

    pub cells: Vec<DentistHmoAuditCell>,

    pub row_total_qty: i64,
    pub row_total_fee: f64,
    pub total_basic_fee: f64,
    pub total_nonbasic_fee: f64,
    pub subtotal_fee: f64,
}
// DentistHmoServiceAuditMatrixResponse is the recommended backend response shape.
// hmos - vector of DentistHmoAuditHmoColumns
// rows - vector of DentistHmoAuditDentistRows
#[derive(Debug, Serialize)]
pub struct DentistHmoServiceAuditMatrixResponse {
    pub start_date: String,
    pub end_date: String,

    // columns
    pub hmos: Vec<DentistHmoAuditHmoColumn>,

    // rows
    pub rows: Vec<DentistHmoAuditDentistRow>,

    // hmo totals
    pub hmo_totals: Vec<DentistHmoAuditHmoTotal>,

    pub grand_total_qty: i64,
    pub grand_total_fee: f64,
    pub grand_total_basic_fee: f64,
    pub grand_total_nonbasic_fee: f64,
}