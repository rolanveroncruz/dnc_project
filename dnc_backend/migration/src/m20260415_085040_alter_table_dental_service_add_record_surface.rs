use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DentalService::Table)
                    .add_column(ColumnDef::new(DentalService::RecordSurface)
                        .boolean()
                        .default(false)
                        .not_null()
                    ).to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DentalService::Table)
                    .drop_column(DentalService::RecordSurface)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}
#[derive(Iden)]
pub enum DentalService{
    Table,
    RecordSurface,
}
