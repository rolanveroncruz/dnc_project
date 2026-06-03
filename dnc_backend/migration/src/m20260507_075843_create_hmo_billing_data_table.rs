use sea_orm_migration::{prelude::*};
use crate::m20260220_082933_create_endorsement_tables::Endorsement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_hmo_billing_data_table(self, manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_hmo_billing_data_table(self, manager).await?;
        Ok(())
    }
}
impl Migration{
    async fn create_hmo_billing_data_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager.create_table(
            Table::create()
                .table(HMOBillingData::Table)
                    .if_not_exists()
                .col(ColumnDef::new(HMOBillingData::Id)
                    .integer()
                    .primary_key()
                    .auto_increment()
                    .not_null()
                )
                .col(ColumnDef::new(HMOBillingData::DateGenerated)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp())
                )
                .col(ColumnDef::new(HMOBillingData::EndorsementId)
                    .integer()
                    .not_null()
                )
                .col(ColumnDef::new(HMOBillingData::RequestKey)
                    .string()
                    .null()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_hmo_billing_data_endorsement_id")
                    .from(HMOBillingData::Table, HMOBillingData::EndorsementId)
                    .to(Endorsement::Table, Endorsement::Id)
                )
                .col(ColumnDef::new(HMOBillingData::MasterListCount)
                    .integer()
                    .default(Expr::val(0))
                )
                .col(ColumnDef::new(HMOBillingData::AddedListCount)
                    .integer()
                    .default(Expr::val(0))
                )
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn drop_hmo_billing_data_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(HMOBillingData::Table).to_owned()).await?;
        Ok(())
    }
}
/*
The billing statement to an HMO is dependent on each endorsement's billing type (annually vs. monthly)
HMOBillngData allows separating computing billing data from generating the actual report.

So each row in the HMOBillingData table represents the master list count for a given endorsement.
From the endorsement, we know the billing type, and the period in question which is the from a month
before DateGenerated to DateGenerated.

We are adding a RequestKey field to allow better tracking of counts. Since counts can be generated theoretically
anytime, when we generate the report for an HMO, we can't just filter on the EndorsementId, as there may
have multiple rows in the HMOBillingData table for the same endorsement (because of successive counts.


 */
#[derive(DeriveIden)]
pub enum HMOBillingData{
    Table,
    Id,
    RequestKey,
    DateGenerated,
    EndorsementId,
    MasterListCount, // total master list members
    AddedListCount   // total added by CSR members
}
