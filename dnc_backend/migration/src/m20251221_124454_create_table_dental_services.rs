use sea_orm_migration::{prelude::*, };
use crate::m20251205_075454_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DentalServiceType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentalServiceType::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(DentalServiceType::Name)
                        .string()
                        .not_null()
                    ).to_owned(),
            )
            .await?;

        Self::create_dental_service_type(manager, "Basic").await?;
        Self::create_dental_service_type(manager, "Special").await?;
        Self::create_dental_service_type(manager, "High-End").await?;


        manager
            .create_table(
                Table::create()
                    .table(DentalService::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentalService::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(DentalService::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentalService::TypeId)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentalService::RecordTooth)
                        .boolean()
                    )
                    .col(ColumnDef::new(DentalService::LastModifiedBy)
                        .integer()
                    )
                    .col(ColumnDef::new(DentalService::LastModifiedOn)
                        .timestamp()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("dental_service_service_type_id_foreign_key")
                            .from(DentalService::Table, DentalService::TypeId)
                            .to(DentalServiceType::Table, DentalServiceType::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    ).foreign_key(
                        ForeignKey::create()
                            .name("dental_service_last_modified_by_foreign_key")
                            .from(DentalService::Table, DentalService::LastModifiedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    ).to_owned(),
            )
            .await?;

        Self::create_dental_service(manager, "Cleaning / Oral Prophylaxis", "Basic", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Checkup/ Consultation", "Basic", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Simple Tooth Extraction", "Basic", true, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Temporary Fillings", "Basic", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Gum Treatment", "Basic", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Recementation of Jacket Crown", "Basic", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Adjustment of Dentures", "Basic", false, "admin@dnc.com.ph").await?;

        Self::create_dental_service(manager, "Additional Cleaning / Oral Prophylaxis", "Special", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Permanent Fillings (per Tooth)", "Special", true, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Permanent Fillings (per Surface)", "Special", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Dental Radiography (Periapical)", "Special", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Dental Radiography (Panoramic)", "Special", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Desensitization", "Special", false, "admin@dnc.com.ph").await?;

        Self::create_dental_service(manager, "Dentures", "High-End", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Odontectomy", "High-End", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Root Canal Treatment", "High-End", false, "admin@dnc.com.ph").await?;
        Self::create_dental_service(manager, "Deep Scaling", "High-End", false, "admin@dnc.com.ph").await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(DentalService::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DentalServiceType::Table).to_owned())
            .await?;
        Ok(())
    }
}

impl Migration {
    async fn create_dental_service_type(manager: &SchemaManager<'_>, name: &str)->Result<(), DbErr>{
        let insert = Query::insert()
            .into_table(DentalServiceType::Table)
            .columns([DentalServiceType::Name])
            .values_panic([Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await
    }
    async fn create_dental_service( manager: &SchemaManager<'_>, name: &str, service_type: &str, record_tooth: bool, last_modified_by: &str )->Result<(), DbErr>{
        use sea_orm_migration::prelude::*;
        use sea_query::{Expr, Query};
        use chrono::Utc;
        let db = manager.get_connection();

        // 1. Resolve DentalServiceType.id by name
        let service_type_id:i32 = {
            let select = Query::select()
                .column(DentalServiceType::Id)
                .from(DentalServiceType::Table)
                .and_where(Expr::col((DentalServiceType::Table, DentalServiceType::Name)).eq(service_type))
                .to_owned();

            let row = db
                .query_one(&select).await?
                .ok_or_else( || DbErr::Custom("DentalServiceType not found".to_string()))?;
            row.try_get ("", &DentalServiceType::Id.to_string())?
        };

        // 2. Resolve User.id by email
        let last_modified_by_user_id:i32 = {
            let select = Query::select()
                .column(User::Id)
                .from(User::Table)
                .and_where(Expr::col((User::Table, User::Email)).eq(last_modified_by))
                .to_owned();

            let row = db
                .query_one(&select).await?
                .ok_or_else( || DbErr::Custom("User '{last_modified_by}' not found".to_string()))?;

            row.try_get ("", &User::Id.to_string())?
        };
        //3. Insert DentalService
        let insert = Query::insert()
            .into_table(DentalService::Table)
            .columns([
                DentalService::Name,
                DentalService::TypeId,
                DentalService::RecordTooth,
                DentalService::LastModifiedBy,
                DentalService::LastModifiedOn
            ])
            .values_panic([
                name.into(),
                service_type_id.into(),
                record_tooth.into(),
                last_modified_by_user_id.into(),
                Utc::now().naive_utc().into()
            ])
            .to_owned();
        db.execute(&insert).await.map_err(|e| DbErr::Custom(format!("{:?}", e)))?;
        Ok(())
    }
}
#[derive(Iden)]
pub enum DentalServiceType{
    Table,
    Id,
    Name,
}
#[derive(Iden)]
pub enum DentalService{
    Table,
    Id,
    Name,
    TypeId,
    RecordTooth,
    LastModifiedBy,
    LastModifiedOn,
}
