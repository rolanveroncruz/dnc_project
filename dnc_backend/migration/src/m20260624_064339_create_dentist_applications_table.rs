use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum DentistApplications {
    Table,
    Id,
    DateSubmitted,
    Name,
    ClinicName,
    ContactNumbers,
    Email,
    PrcLicenseFilePath,
    Bir2303FilePath,
    Status,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DentistApplications::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistApplications::Id)
                        .integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::DateSubmitted)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .col(ColumnDef::new(DentistApplications::Name)
                        .text()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::ClinicName)
                        .text()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::ContactNumbers)
                        .text()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::Email)
                        .text()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::PrcLicenseFilePath)
                        .text().not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::Bir2303FilePath)
                    .text().not_null()
                    )
                    .col(ColumnDef::new(DentistApplications::Status)
                        .text()
                        .not_null()
                        .default("new".to_owned())
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(DentistApplications::Table)
                    .if_exists()
                    .to_owned()
            ).await?;
        Ok(())
    }
}
