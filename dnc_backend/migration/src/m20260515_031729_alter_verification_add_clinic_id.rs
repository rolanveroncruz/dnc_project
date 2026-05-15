use sea_orm_migration::{prelude::*};
use crate::m20260126_063012_create_tables_dental_clinic::DentalClinic;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_column( ColumnDef::new(Verification::DentalClinicId)
                        .integer()
                        .not_null()
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_verification_dental_clinic_id")
                            .from_tbl(Verification::Table)
                            .from_col(Verification::DentalClinicId)
                            .to_tbl(DentalClinic::Table)
                            .to_col(DentalClinic::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_foreign_key(Alias::new("fk_verification_dental_clinic_id"))
                    .drop_column(Verification::DentalClinicId)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Verification {
    Table,
    DentalClinicId,
}
