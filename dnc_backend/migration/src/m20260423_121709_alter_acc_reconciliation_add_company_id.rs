use sea_orm_migration::{prelude::*};
use crate::m20260220_082933_create_endorsement_tables::EndorsementCompany;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, we add the company_id column.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .add_column(
                        ColumnDef::new(AccReconciliation::CompanyId)
                            .integer()
                            .not_null()
                    )
                    .to_owned()
            ).await?;
        // Then, we add FK to the endorsement_company table.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_acc_reconciliation_table_company_id")
                            .from_tbl(AccReconciliation::Table)
                            .from_col(AccReconciliation::CompanyId)
                            .to_tbl(EndorsementCompany::Table)
                            .to_col(EndorsementCompany::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, we drop FK to the endorsement_company table.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .drop_foreign_key("fk_acc_reconciliation_table_company_id")
                    .to_owned(),
            ).await?;
        // Then, we drop the company_id column from the table.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .drop_column(AccReconciliation::CompanyId)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
pub enum AccReconciliation {
    Table,
    CompanyId,
}
