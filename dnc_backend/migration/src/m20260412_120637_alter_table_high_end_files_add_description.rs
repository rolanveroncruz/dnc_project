use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(HighEndFiles::Table)
                    .add_column( ColumnDef::new(HighEndFiles::Description)
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
                    .drop_column(HighEndFiles::Description)
                    .to_owned(),
            ).await?;
        Ok(())

    }
}
#[derive(Iden)]
pub enum HighEndFiles{
    Table,
    Description
}
