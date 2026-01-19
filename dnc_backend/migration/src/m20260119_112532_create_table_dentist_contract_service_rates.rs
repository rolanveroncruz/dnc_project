use sea_orm_migration::{prelude::*, };
use crate::m20251221_124454_create_table_dental_services::DentalService;
use crate::m20260119_112338_create_table_dentist_contract::DentistContract;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(DentistContractServiceRates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistContractServiceRates::Id)
                             .integer()
                             .primary_key()
                             .auto_increment()
                             .not_null()
                    )
                    .col(ColumnDef::new(DentistContractServiceRates::DentistContractId)
                            .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_contract_service_rate__dentist_contract_id")
                        .from(DentistContractServiceRates::Table, DentistContractServiceRates::DentistContractId)
                        .to(DentistContract::Table, DentistContract::Id)
                    )
                    .col(ColumnDef::new(DentistContractServiceRates::ServiceId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_contract_service_rate__service_id")
                        .from(DentistContractServiceRates::Table, DentistContractServiceRates::ServiceId)
                        .to(DentalService::Table, DentalService::Id))
                    .col(ColumnDef::new(DentistContractServiceRates::Rate)
                        .float()
                        .not_null()
                        .default(0.0)
                    )
                    .to_owned(),
            )
            .await?;

            Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(DentistContractServiceRates::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum DentistContractServiceRates{
    Table,
    Id,
    DentistContractId,
    ServiceId,
    Rate,
}
