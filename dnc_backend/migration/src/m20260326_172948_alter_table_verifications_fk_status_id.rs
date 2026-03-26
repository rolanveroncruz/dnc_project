use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop old FK: verification.status_id -> verification_status.id
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_foreign_key(Alias::new("fk_verification_table_status_id"))
                    .to_owned(),
            )
            .await?;

        // Recreate FK: verification.status_id -> verification_status.int_code
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_verification_table_status_id")
                            .from_tbl(Verification::Table)
                            .from_col(Verification::StatusId)
                            .to_tbl(VerificationStatus::Table)
                            .to_col(VerificationStatus::IntCode)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop FK to int_code
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_foreign_key(Alias::new("fk_verification_table_status_id"))
                    .to_owned(),
            )
            .await?;

        // Restore FK to id
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_verification_table_status_id")
                            .from_tbl(Verification::Table)
                            .from_col(Verification::StatusId)
                            .to_tbl(VerificationStatus::Table)
                            .to_col(VerificationStatus::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    StatusId,
}

#[derive(DeriveIden)]
enum VerificationStatus {
    Table,
    Id,
    IntCode,
}