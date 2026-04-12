use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_column( ColumnDef::new(Verification::ToothId)
                        .string()
                        .null()
                    ).to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_column(Verification::ToothId)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}


// We'll only put in the column we'll be adding
#[derive(Iden)]
enum Verification {
    Table,
    ToothId,
}