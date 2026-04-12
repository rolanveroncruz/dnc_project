use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(HighEndFiles::Table)
                    .add_column( ColumnDef::new(HighEndFiles::OriginalFilename)
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
                    .table(HighEndFiles::Table)
                    .drop_column(HighEndFiles::OriginalFilename)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum HighEndFiles{
    Table,
    OriginalFilename,
}
