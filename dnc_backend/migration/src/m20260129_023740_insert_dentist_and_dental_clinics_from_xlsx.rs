use sea_orm_migration::{prelude::*};
use calamine::{open_workbook_from_rs, Data, Reader, Xlsx};
use std::io::Cursor;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Self::insert_dentists_and_clinics_from_xlsx(_manager)?;
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
    first_name: String,
    middle_name: String,
    last_name: String,
    clinic_name: String,
    clinic_address: String,
    clinic_city: String,
    clinic_province: String,
    clinic_region: String,
    clinic_zip: String,
    clinic_contact: String,
    clinic_schedule: String,
    dentist_position: String,
}
impl DentistRow{
    fn from_row( row: &[Data])->Option<Self>{
        let get = |i:usize| row.get(i).map(cell_to_string).unwrap_or_default();

        if row.iter().all(|c| matches!(c, Data::Empty)) {
            return None;
        }
        let dentist = Self{
            contract: get(0),
            first_name: get(2),
            middle_name: get(3),
            last_name:get(4),
            clinic_name:get(5),
            clinic_address: get(6),
            clinic_city: get(7),
            clinic_province:get(8),
            clinic_region:get(9),
            clinic_zip: get(10),
            clinic_contact:get(11),
            clinic_schedule:get(12),
            dentist_position:get(13),
        };

        Some(dentist)

    }
}
impl Migration{
    fn insert_dentists_and_clinics_from_xlsx(_manager: &SchemaManager) -> Result<(), DbErr> {
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
        println!("{:?}", dentists);


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
