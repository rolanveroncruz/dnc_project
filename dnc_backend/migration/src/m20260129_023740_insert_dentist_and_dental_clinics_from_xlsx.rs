use sea_orm_migration::{prelude::*};
use calamine::{open_workbook_from_rs, Data, Reader, Xlsx};
use std::io::Cursor;
use crate::m20260126_063012_create_tables_dental_clinic::{City, DentalClinic, Region};
use crate::m20260126_063012_create_tables_dental_clinic::Province;
use sea_query::{Expr, OnConflict, Query};
use crate::m20260119_112338_create_table_dentist_contract::DentistContract;
use crate::m20260126_161604_create_table_dentists::{Dentist, DentistClinic, Position};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Self::insert_dentists_and_clinics_from_xlsx(_manager).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct DentistRow{
    contract: String,
    given_name: String,
    middle_name: String,
    last_name: String,
    clinic_name: String,
    clinic_address: String,
    clinic_city: String,
    clinic_province: String,
    clinic_region: String,
    clinic_zip: String,
    clinic_contact_numbers: String,
    clinic_schedule: String,
    dentist_position: String,
    clinic_email: String,
}

impl DentistRow{
    fn from_row( row: &[Data])->Option<Self>{
        let get = |i:usize| row.get(i).map(cell_to_string).unwrap_or_default();

        if row.iter().all(|c| matches!(c, Data::Empty)) {
            return None;
        }
        let dentist = Self{
            contract: get(0),
            given_name: get(2),
            middle_name: get(3),
            last_name:get(4),
            clinic_name:get(5),
            clinic_address: get(6),
            clinic_city: get(7),
            clinic_province:get(8),
            clinic_region:get(9),
            clinic_zip: get(10),
            clinic_contact_numbers:get(11),
            clinic_schedule:get(12),
            dentist_position:get(13),
            clinic_email: get(15),
        };
        Some(dentist)
    }
}
impl Migration{
    async fn insert_dentists_and_clinics_from_xlsx(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        static DENTISTS_2026: &[u8] = include_bytes!("../../data_input/dentists_2026.xlsx");

        let mut workbook:Xlsx<_> = open_workbook_from_rs(Cursor::new(DENTISTS_2026))
            .map_err(|e| DbErr::Custom(format!("Failed to open workbook : {e}")))?;

        let sheet_name = workbook
            .sheet_names()
            .get(0)
            .cloned()
            .ok_or_else(|| DbErr::Custom("Failed to get sheet name".to_string()))?;

        let range = workbook
            .worksheet_range(&*sheet_name)
            .map_err(|e| DbErr::Custom(format!("Failed to get worksheet range: {e}")))?;


        let dentists:Vec<DentistRow> = range
            .rows()
            .skip(1)
            .filter_map(DentistRow::from_row)
            .collect();

         for (i,dentist) in dentists.iter().enumerate() {
             if (i+1)%50==0{
                 println!("inserted {}/{} dentists",i+1, dentists.len());
             }
             Self::insert_dentist_row(&dentist, manager).await?;
         }


        Ok(())
    }

    async fn insert_dentist_row(d_row: &DentistRow, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // ---- 1) REGION ----
        let region_name = clean(&d_row.clinic_region);
        let region_id = get_or_insert_region(db, &region_name).await?;
        println!("region_id={region_id}");

        // ---- 2) PROVINCE (unique by name+region_id) ----
        let province_name = clean(&d_row.clinic_province);
        let province_id = get_or_insert_province(db, &province_name, region_id).await?;
        println!("province_id={province_id}");

        // ---- 3) CITY (unique by name+province_id) ----
        let city_name = clean(&d_row.clinic_city);
        let city_id = get_or_insert_city(db, &city_name, province_id).await?;
        println!("city_id={city_id}");

        //------4) DENTIST CONTRACT ----
        let dentist_contract_name = clean(&d_row.contract);
        let dentist_contract_id = get_or_insert_dentist_contract(db, &dentist_contract_name).await?;
        println!("dentist_contract_id={dentist_contract_id}");

        //-----5) DENTAL CLINIC ----
        let clinic_name = clean(&d_row.clinic_name);
        let clinic_address = clean(&d_row.clinic_address);
        let clinic_city_id = Some(city_id);
        let clinic_zip = Some(d_row.clinic_zip.as_str());
        let clinic_contact = Some(d_row.clinic_contact_numbers.as_str());
        let clinic_schedule = Some(d_row.clinic_schedule.as_str());
        let clinic_email = Some(d_row.clinic_email.as_str());

        let dental_clinic_id = get_or_insert_dental_clinic(
            db, &clinic_name, &clinic_address, clinic_city_id,
            clinic_zip,clinic_contact, clinic_schedule, clinic_email
        ).await?;
        println!("dental_clinic_id={dental_clinic_id}");


        //---6) DENTIST ----
        let dentist_given_name = clean(&d_row.given_name);
        let dentist_middle_name = clean(&d_row.middle_name);
        let dentist_last_name = clean(&d_row.last_name);

        //---7) DENTIST_CLINIC_POSITION
        let dentist_position_raw = clean(&d_row.dentist_position);
        let dentist_position_name = map_xlsx_position_to_position_name(&dentist_position_raw)?;
        let dentist_position_id = get_position_id_by_name(db, &dentist_position_name).await?;


        insert_dentist(db, &dentist_given_name, &dentist_middle_name, &dentist_last_name,
                       dentist_contract_id, dental_clinic_id, dentist_position_id).await?;
        println!("dentist inserted:{dentist_given_name} {dentist_middle_name} {dentist_last_name}");
        Ok(())
    }
}
fn cell_to_string(cell:&Data) ->String{
    match cell{
        Data::Empty => "".to_string(),
        Data::String(s) => s.to_string(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => dt.to_string(),
        Data::DurationIso(s)=>s.clone(),
        Data::DateTimeIso(s)=>s.clone(),
        Data::Error(e) => e.to_string(),
    }
}

// clean trims and normalizes whitespace.
// it splits a string on the whitespaces then joins them back together with a single space.
fn clean(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ").trim().to_string()
}

async fn get_or_insert_region(db: &impl ConnectionTrait, name: &str) -> Result<i32, DbErr> {
    let insert = Query::insert()
        .into_table(Region::Table)
        .columns([Region::Name])
        .values_panic([name.into()])
        .on_conflict(OnConflict::column(Region::Name).do_nothing().to_owned())
        .to_owned();

        db.execute(&insert).await?;
    println!("inserted region={name}");

    let select = Query::select()
        .column(Region::Id)
        .from(Region::Table)
        .and_where(Expr::col(Region::Name).eq(name))
        .limit(1)
        .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get region id for {name}")))?;

    Ok(row.try_get("", "id")?)
}

async fn get_or_insert_province(db: &impl ConnectionTrait, name: &str, region_id: i32) -> Result<i32, DbErr> {
    let insert = Query::insert()
        .into_table(Province::Table)
        .columns([Province::Name, Province::RegionId])
        .values_panic([name.into(), region_id.into()])
        .on_conflict(OnConflict::columns([Province::Name, Province::RegionId]).do_nothing().to_owned())
        .to_owned();

    db.execute(&insert).await?;
    println!("inserted province={name}");

    let select = Query::select()
        .column(Province::Id)
        .from(Province::Table)
        .and_where(Expr::col(Province::Name).eq(name))
        .and_where(Expr::col(Province::RegionId).eq(region_id))
        .limit(1)
        .to_owned();
    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get province id for {name}")))?;
    Ok(row.try_get("", "id")?)
}

async fn get_or_insert_city(db: &impl ConnectionTrait, name: &str, province_id: i32) -> Result<i32, DbErr> {

    let insert = Query::insert()
        .into_table(City::Table)
        .columns([City::Name, City::ProvinceId])
        .values_panic([name.into(), province_id.into()])
        .on_conflict(OnConflict::columns([City::Name, City::ProvinceId]).do_nothing().to_owned())
        .to_owned();
    db.execute(&insert).await?;
    println!("inserted city={name}");

    let select = Query::select()
        .column(City::Id)
        .from(City::Table)
        .and_where(Expr::col(City::Name).eq(name))
        .and_where(Expr::col(City::ProvinceId).eq(province_id))
        .limit(1)
    .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get city id for {name}")))?;
    Ok(row.try_get("", "id")?)
}


async fn get_or_insert_dentist_contract(db: &impl ConnectionTrait, name: &str) -> Result<i32, DbErr> {
    let insert = Query::insert()
        .into_table(DentistContract::Table)
        .columns([DentistContract::Name, DentistContract::LastModifiedBy])
        .values_panic([name.into(), "system".into()])
        .on_conflict(OnConflict::column(DentistContract::Name).do_nothing().to_owned())
        .to_owned();

    db.execute(&insert).await?;
    println!("inserted dentist_contract={name}");

    let select = Query::select()
        .column(DentistContract::Id)
        .from(DentistContract::Table)
        .and_where(Expr::col(DentistContract::Name).eq(name))
        .limit(1)
        .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get dentist_contract id for {name}")))?;

    Ok(row.try_get("", "id")?)

}

async fn get_or_insert_dental_clinic(db: &impl ConnectionTrait,
                                     clinic_name: &str, clinic_address: &str,
                                     clinic_city_id:Option<i32>, clinic_zip: Option<&str>,
                                     clinic_contact: Option<&str>, clinic_schedule: Option<&str>,
                                     clinic_email: Option<&str>
) -> Result<i32, DbErr> {
    let insert = Query::insert()
        .into_table(DentalClinic::Table)
        .columns([DentalClinic::Name,
            DentalClinic::Address,
            DentalClinic::CityId,
            DentalClinic::ZipCode,
            DentalClinic::ContactNumbers,
            DentalClinic::Schedule,
            DentalClinic::Email

        ])
        .values_panic([
            clinic_name.into(),
            clinic_address.into(),
            clinic_city_id.into(),
            clinic_zip.into(),
            clinic_contact.into(),
            clinic_schedule.into(),
            clinic_email.into()])
        .on_conflict(OnConflict::columns([
            DentalClinic::Name,
            DentalClinic::Address,
            DentalClinic::CityId,
            DentalClinic::ZipCode])
            .do_nothing()
            .to_owned()
        )
        .to_owned();

    db.execute(&insert).await?;
    println!("inserted dental_clinic={clinic_name}");

    let select = Query::select()
        .column(DentalClinic::Id)
        .from(DentalClinic::Table)
        .and_where(Expr::col(DentalClinic::Name).eq(clinic_name))
        .and_where(Expr::col(DentalClinic::Address).eq(clinic_address))
        .and_where(Expr::col(DentalClinic::CityId).eq(clinic_city_id))
        .and_where(Expr::col(DentalClinic::ZipCode).eq(clinic_zip))
        .limit(1)
        .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get dental_clinic id for {clinic_name}")))?;

    Ok(row.try_get("", "id")?)
}


async fn insert_dentist(db: &impl ConnectionTrait,
                        given_name:&str, middle_name: &str, last_name: &str,
                        dental_contract_id: i32, dental_clinic_id: i32, position_id: i32) -> Result<(), DbErr> {
    let insert = Query::insert()
        .into_table(Dentist::Table)
        .columns([Dentist::AccreDentistContractId,
        Dentist::GivenName,
        Dentist::MiddleName,
        Dentist::LastName,
        ])
        .values_panic([dental_contract_id.into(),
                            given_name.into(),
                            middle_name.into(),
                            last_name.into(),
                            ])
        .to_owned();
    db.execute(&insert).await?;

    let select = Query::select()
        .from(Dentist::Table)
        .column(Dentist::Id)
        .and_where(Expr::col(Dentist::GivenName).eq(given_name))
        .and_where(Expr::col(Dentist::MiddleName).eq(middle_name))
        .and_where(Expr::col(Dentist::LastName).eq(last_name))
        .limit(1)
        .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to get dentist id for {given_name} {middle_name} {last_name}")))?;

   let dentist_id: i32 = row.try_get("", "id")?;

   let insert_dentist_clinic = Query::insert()
       .into_table(DentistClinic::Table)
       .columns([DentistClinic::DentistId, DentistClinic::ClinicId, DentistClinic::PositionId])
       .values_panic([dentist_id.into(), dental_clinic_id.into(), position_id.into()])
       .to_owned();

    db.execute(&insert_dentist_clinic).await?;

    Ok(())
}

fn map_xlsx_position_to_position_name( raw:&str)->Result<String, DbErr>{
    // normalize comparison
    let lowered = raw.trim().to_lowercase();

    let mapped = match lowered.as_str() {
        "Owner" => "Principal",
        "owner" => "Principal",
        "Associate dentist" => "Associate",
        "associate dentist" => "Associate",
        "" => {
            return Err(DbErr::Custom(
                "Empty clinic position in XLSX (expected 'Owner' or 'Associate dentist')".to_string(),
            ));
        }
        other => {
            return Err(DbErr::Custom(format!(
                "Unknown clinic position in XLSX: '{other}' (expected 'Owner' or 'Associate dentist')"
            )));
        }
    };

    Ok(mapped.to_string())
}
// ANNOTATED CHANGE: fetch Position.id by Position.name (NO INSERT; must already exist)
async fn get_position_id_by_name(db: &impl ConnectionTrait, name: &str) -> Result<i32, DbErr> {
    let select = Query::select()
        .column(Position::Id)
        .from(Position::Table)
        .and_where(Expr::col(Position::Name).eq(name))
        .limit(1)
        .to_owned();

    let row = db.query_one(&select).await?
        .ok_or_else(|| DbErr::Custom(format!("Failed to find Position.id for name='{name}'")))?;

    Ok(row.try_get("", "id")?)
}
