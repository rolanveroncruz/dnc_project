use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_report_type_table(manager).await?;
        Self::create_report_type_type(manager, "HMO Billing").await?;
        Self::create_report_type_type(manager, "Dentist Claims").await?;
        Self::create_report_type_type(manager, "Dentist Retainers").await?;

        Self::create_generated_report_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_generated_report_table(manager).await?;
        Self::drop_report_type_table(manager).await?;
        Ok(())

    }
}

impl Migration {
    async fn create_report_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReportType::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(ReportType::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(ReportType::Name)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    async fn drop_report_type_table(manager: &SchemaManager<'_>)-> Result<(), DbErr>{
        manager
            .drop_table(Table::drop().table(ReportType::Table).to_owned())
            .await?;
        Ok(())
    }
    async fn create_report_type_type(manager: &SchemaManager<'_>, type_name: &str)-> Result<(), DbErr>{
        let insert = Query::insert()
            .into_table(ReportType::Table)
            .columns([ ReportType::Name])
            .values_panic([Expr::val(type_name)])
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())

    }

    async fn create_generated_report_table(manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .create_table(
                Table::create()
                    .table(GeneratedReport::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GeneratedReport::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(GeneratedReport::ReportTypeId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_generated_report_report_type_id")
                        .from(GeneratedReport::Table, GeneratedReport::ReportTypeId)
                        .to(ReportType::Table, ReportType::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(GeneratedReport::FileName)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(GeneratedReport::DateGenerated)
                        .timestamp_with_time_zone()
                    )
                        .to_owned()
            ).await?;
        Ok(())
    }
    async fn drop_generated_report_table(manager: &SchemaManager<'_>)-> Result<(), DbErr>{
        manager
            .drop_table(Table::drop().table(GeneratedReport::Table).to_owned()).await?;
        Ok(())
    }

}
/*
Report Types provide a numeric value for the types of reports we are generating.
1- HMO Billing, 2- Dentist Claims, 3- Dentist Retainers
 */
#[derive(DeriveIden)]
pub enum ReportType{
    Table,
    Id,
    Name,
}
/*
This table records generated reports.
 */
#[derive(DeriveIden)]
pub enum GeneratedReport{
    Table,
    Id,
    ReportTypeId,
    FileName,
    DateGenerated,
}

