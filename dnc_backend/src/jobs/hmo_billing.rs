use sea_orm::{PaginatorTrait, QueryOrder};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use chrono::{NaiveDate, Months, Utc};
use tracing::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, JoinType, QuerySelect, RelationTrait, Set};
use crate::AppState;

use crate::entities::{dental_service,
                      endorsement, endorsement_company,endorsement_counts,
                      generated_report,
                      hmo, hmo_billing_data,
                      master_list, master_list_member};
use umya_spreadsheet;
use uuid::Uuid;

/// Generate HMO Billing Reports for each HMO
///
/// It does this by computing the number of master_list_members for each endorsement, saving it to the DB;
/// then iterating through all HMOs and generating the Excel report for each HMO
pub async fn generate_hmo_billing_reports(
    state: AppState,
    start_date: Option<NaiveDate>,
    end_date: NaiveDate,
)-> anyhow::Result<()> {

   let request_key = Uuid::new_v4().to_string();

    let db : &DatabaseConnection = &state.db;
    let actual_start_date = match start_date {
        Some(date) => date,
        None => end_date
            .checked_sub_months(Months::new(1)).ok_or_else(|| anyhow::anyhow!("Could not calculate start date"))?,
    };
    info!(target: "jobs",
        "generate_hmo_billing_reports() started with request_key {} for period {}-{}",
        request_key,
        actual_start_date.format("%m/%d/%Y"),
        end_date.format("%m/%d/%Y")
    );

    //---1. Generate Billing Data per endorsement
    let endorsements = endorsement::Entity::find()
        .all(db)
        .await?;
    for endorsement in endorsements{
        generate_billing_data_for_endorsement(state.clone(), request_key.clone(), endorsement.id, actual_start_date, end_date).await?;
    }

    //---2. Generate Billing Report per HMO
    let hmos = hmo::Entity::find()
        .all(db)
        .await?;
    for hmo in hmos{
        generate_billing_report_for_hmo(state.clone(),request_key.clone(), hmo.id, actual_start_date, end_date).await?;
    }
    Ok(())
}



/// For each endorsement, count the master_list_members for the period defined by start_date and end_date.
/// The method of counting differs depending on whether its endorsement_billing_period_type is annual or monthly.
async fn generate_billing_data_for_endorsement(
    state: AppState,
    request_key: String,
    endorsement_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,

)-> anyhow::Result<()> {
    let db : &DatabaseConnection = &state.db;
    let endorsement = endorsement::Entity::find_by_id(endorsement_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Could not find endorsement with id {}", endorsement_id))?;
    let billing_period_type_id = endorsement.endorsement_billing_period_type_id;
    let master_list_member_count = match billing_period_type_id {
        1 => {
            count_annual_master_list_members_for_endorsement(state.clone(), endorsement_id, start_date, end_date).await?
        },
        2 => {
            count_monthly_master_list_members_for_endorsement(state.clone(), endorsement_id, start_date, end_date).await?
        },
        _ => {
            return Err(anyhow::anyhow!("Invalid billing period type id {}", billing_period_type_id));
        }
    };
    let new_billing_data = hmo_billing_data::ActiveModel {
        id: Default::default(),
        date_generated: Set(Utc::now().fixed_offset()),
        request_key: Set(Some(request_key)),
        endorsement_id: Set(endorsement_id),
        master_list_count: Set(Some(master_list_member_count.master_list_members_count as i32)),
        added_list_count: Set(Some(master_list_member_count.added_counts as i32))

    };
    let _inserted_billing_data = new_billing_data.insert(db).await?;

    Ok(())
}


// region MLM: counters
struct MLMCounts{
    master_list_members_count: u64,
    added_counts: u64,
}
/// Endorsements with Annual Billing Period only count the master_list_members
/// whose master_list was uploaded within that period, plus members that were added in that period.
async fn count_annual_master_list_members_for_endorsement(
    state: AppState,
    endorsement_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
)-> anyhow::Result<MLMCounts> {
    let db : &DatabaseConnection = &state.db;
    let mlm_count = master_list_member::Entity::find()
        .join(
            JoinType::InnerJoin,
            master_list_member::Relation::MasterList.def(),
        )
        .filter(master_list::Column::EndorsementId.eq(endorsement_id))
        .filter(master_list::Column::UploadDate.gte(start_date))
        .filter(master_list::Column::UploadDate.lte(end_date))
        .count(db)
        .await?;

    let added_count = master_list_member::Entity::find()
        .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
        .filter(master_list_member::Column::LastEditedDate.gte(start_date))
        .filter(master_list_member::Column::LastEditedDate.lte(end_date))
        .count(db)
        .await?;

    let total_count = MLMCounts{
        master_list_members_count: mlm_count,
        added_counts: added_count,
    };
    Ok(total_count)
}

/// Endorsements with Monthly Billing Period count all total master_list_members
/// for that period.
async fn count_monthly_master_list_members_for_endorsement(
    state: AppState,
    endorsement_id: i32,
    _start_date: NaiveDate,
    _end_date: NaiveDate,
)-> anyhow::Result<MLMCounts> {
    let db : &DatabaseConnection = &state.db;
    let mlm_count = master_list_member::Entity::find()
        .filter(master_list_member::Column::MasterListId.is_not_null())
        .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
        .count(db)
        .await?;

    let add_count = master_list_member::Entity::find()
    .filter(master_list_member::Column::EndorsementId.eq(endorsement_id))
    .filter(master_list_member::Column::MasterListId.is_null())
    .count(db)
    .await?;

    let total_count = MLMCounts{
        master_list_members_count: mlm_count,
        added_counts: add_count,
    };
    Ok(total_count)
}

// endregion: MLM counters

/// generate_billing_report_for_hmo() creates the Excel report for the HMO.
///
async fn generate_billing_report_for_hmo(
    state: AppState,
    request_key: String,
    hmo_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
)-> anyhow::Result<()> {

    let db : &DatabaseConnection = &state.db;
    let the_hmo = hmo::Entity::find_by_id(hmo_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Could not find HMO with id {}", hmo_id))?;

    //----- Get all billing data for
    let hmo_billing_data_rows = hmo_billing_data::Entity::find()
    .join(
        JoinType::InnerJoin,
        hmo_billing_data::Relation::Endorsement.def(),
    )
        .filter(endorsement::Column::HmoId.eq(hmo_id))
        .filter(hmo_billing_data::Column::RequestKey.eq(Some(request_key.clone())))
        .all(db)
        .await?;

    //--- Generate the Excel report only for HMOs that aren't empty.
    if hmo_billing_data_rows.is_empty() {
        info!(target: "jobs",
            "generate_billing_report_for_hmo() skipped for HMO id:{}({}) because it has no billing data",
            hmo_id,
            the_hmo.short_name
        );
        return Ok(());
    }
    let the_filename = write_hmo_billing_to_spreadsheet( state.clone(), &the_hmo.short_name, hmo_billing_data_rows, end_date).await?;


    let generated_report_record = generated_report::ActiveModel{
        id: Default::default(),
        report_type_id: Set(1),
        file_name: Set(the_filename.clone()),
        date_generated: Set(Some(Utc::now().fixed_offset())),
    };
    let _ = generated_report_record.insert(db).await?;
    Ok(())
}


/// write_hmo_billing_to_spreadsheet() does the actual work of writing data to an Excel spreadsheet.
///

async fn write_hmo_billing_to_spreadsheet(
    state:AppState,
    hmo_name:&str,
    billing_data: Vec<hmo_billing_data::Model>,
    end_date: NaiveDate)
    -> anyhow::Result<String> {
    let db : &DatabaseConnection = &state.db;

    info!(target: "jobs", "write_hmo_billing_to_spreadsheet() started for HMO {} with {} rows", hmo_name, billing_data.len());
    //----1. Set the path to template XLSX
    let template_path = "billing_templates/HMO_Billing_Template.xlsx";

    //----2. Load workbook
    let mut book = umya_spreadsheet::reader::xlsx::read(template_path)
        .map_err(|e| anyhow::anyhow!("Failed to read XLSX template: {}", e))?;

    // ✅ 3. Pick worksheet
    let sheet = book
        .get_sheet_by_name_mut("summary")
        .ok_or_else(|| anyhow::anyhow!( "Worksheet 'Sheet1' not found in template:"))?;

    // 4. Write Title stuff
    sheet.get_cell_mut("D10").set_value(hmo_name);
    let month = end_date.format("%B").to_string();
    let year = end_date.format("%Y").to_string();
    let doc_name = format!("SUMMARY OF BILLING - {}{}",month, year);
    sheet.get_cell_mut("D12").set_value(doc_name);


    // 5.Write the data
    let start_row: u32 = 17;
    for (index, row) in billing_data.iter().enumerate() {
        let excel_row = start_row + index as u32;
        if index > 0 {
            sheet.insert_new_row(&excel_row, &1);
        }
        let endorsement_id = row.endorsement_id;


        let endorsement = endorsement::Entity::find_by_id(endorsement_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Could not find endorsement with id {}", endorsement_id))?;

        let company_id = endorsement.endorsement_company_id;
        let company = endorsement_company::Entity::find_by_id(company_id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Could not find endorsement company with id {}", company_id))?;

        // B - index
        // D - name of company
        let company_name =company.name;
        // E - agreement corp number
        let agreement_corp_number = endorsement.agreement_corp_number;
        // F - total master list members
        let total_master_list_members = row.master_list_count.unwrap_or_default() + row.added_list_count.unwrap_or_default();
        // G - billing period type
        let billing_period_str = match endorsement.endorsement_billing_period_type_id {
            1 => "Annual",
            2 => "Monthly",
            _ => "Unknown",
        };
        // H - dental benefits
        let dental_benefits = get_dental_benefits_string_from_endorsement(state.clone(),endorsement.id).await.unwrap_or_default();
        // I - effectivity period
        let effectivity_period = format!("{} - {}", endorsement.date_start.format("%m/%d/%Y"),endorsement.date_end.format("%m/%d/%Y") );
        // J - retainer fee
        // K- 12% VAT
        // L= J+K
        // M = L * F
        sheet
            .get_cell_mut(format!("B{}", excel_row))
            .set_value(index.to_string());

        sheet
            .get_cell_mut(format!("D{}", excel_row))
            .set_value(company_name.clone());

        sheet
            .get_cell_mut(format!("E{}", excel_row))
            .set_value(agreement_corp_number.unwrap_or_default());

        sheet
            .get_cell_mut(format!("F{}", excel_row))
            .set_value(total_master_list_members.to_string());

        sheet
            .get_cell_mut(format!("G{}", excel_row))
            .set_value(billing_period_str);

        sheet
            .get_cell_mut(format!("H{}", excel_row))
            .set_value(dental_benefits.clone());

        sheet
            .get_cell_mut(format!("I{}", excel_row))
            .set_value(effectivity_period);

        sheet
            .get_cell_mut(format!("J{}", excel_row))
            .set_value(endorsement.retainer_fee.unwrap_or_default().to_string());

        info!(target: "jobs",
        "writing row {} for company {} with total_master_list_members {} and dental_benefits '{}'",
        excel_row,
            company_name.clone(),
            total_master_list_members,
            dental_benefits.clone());

    }

    let the_filename = format!("{}_HMO_Billing_{}.xlsx", hmo_name, Utc::now().format("%Y-%m-%d"));
    let full_filename = format!("generated_reports/{}", the_filename);

    info!(target: "jobs", " writing report for {} to {}", hmo_name,full_filename );
    umya_spreadsheet::writer::xlsx::write(&book, &full_filename)
        .map_err(|e| anyhow::anyhow!("Failed to write XLSX: {}", e))?;


    Ok(the_filename)
}

async fn get_dental_benefits_string_from_endorsement(
    state: AppState,
    endorsement_id: i32,
)  -> anyhow::Result<String> {
    let db : &DatabaseConnection = &state.db;

    let rows = endorsement_counts::Entity::find()
        .find_also_related(dental_service::Entity)
        .filter(endorsement_counts::Column::EndorsementId.eq(endorsement_id))
        .filter(endorsement_counts::Column::Count.gt(0))
        .filter(dental_service::Column::TypeId.ne(1))
        .order_by_asc(dental_service::Column::SortIndex)
        .order_by_asc(dental_service::Column::Name)
        .all(db)
        .await?;
    info!(target: "jobs",
        "get_dental_benefits_string_from_endorsement id:{} found {} rows",
        endorsement_id,
        rows.len()
    );

    let mut benefits = String::from("Basic");
    for (count_row, dental_service) in rows{
        if let Some(service) = dental_service{
            benefits.push_str(&format!(" (+) {} ({}) ", service.name, count_row.count));
        }

    }
    Ok(benefits)

}