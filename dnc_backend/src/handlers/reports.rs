use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use crate::entities::generated_report;

#[derive(Debug, Serialize)]
pub struct GeneratedBillingReportResponse {
    pub id: i32,
    pub report_type_id: i32,
    pub file_name: String,
    pub date_generated: Option<String>,
}
pub async fn get_bill_reports(
    db: &DatabaseConnection,
    report_type_id: i32,
) -> anyhow::Result<Vec<GeneratedBillingReportResponse>> {
    let rows = generated_report::Entity::find()
        .filter(generated_report::Column::ReportTypeId.eq(report_type_id))
        .all(db)
        .await?;

    let response = rows
        .into_iter()
        .map(|row| GeneratedBillingReportResponse {
            id: row.id,
            report_type_id: row.report_type_id,
            file_name: row.file_name,
            date_generated: row
                .date_generated
                .map(|dt| dt.format("%B %-d, %Y").to_string()),
        })
        .collect();

    Ok(response)
}