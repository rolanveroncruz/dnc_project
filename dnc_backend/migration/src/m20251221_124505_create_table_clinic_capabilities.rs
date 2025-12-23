use sea_orm_migration::{prelude::*, };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClinicCapability::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ClinicCapability::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(ClinicCapability::Name)
                        .string()
                        .not_null()
                    ).to_owned()
            ).await?;
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClinicCapability::Table).to_owned())
            .await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
pub enum ClinicCapability{
    Table,
    Id,
    Name,
}
