use sea_orm_migration::{prelude::*};
use crate::m20260220_082933_create_endorsement_tables::Endorsement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndorsementBillingRule::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EndorsementBillingRule::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .col(ColumnDef::new(EndorsementBillingRule::EndorsementId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_endorsement_billing_rule_endorsement_id")
                        .from(EndorsementBillingRule::Table, EndorsementBillingRule::EndorsementId)
                        .to(Endorsement::Table, Endorsement::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(EndorsementBillingRule::MinCount)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(EndorsementBillingRule::MaxCount)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(EndorsementBillingRule::Rate)
                        .decimal()
                        .not_null()
                    )
                        .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EndorsementBillingRule::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum EndorsementBillingRule{
    Table,
    Id,
    EndorsementId,
    MinCount,
    MaxCount,
    Rate
}