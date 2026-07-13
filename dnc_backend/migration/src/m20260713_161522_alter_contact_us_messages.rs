use sea_orm_migration::{prelude::*};

#[derive(DeriveIden)]
enum ContactUsMessages {
    Table,
    CompanyAddress,
    Designation
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_contact_us_messages_add_columns(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_contact_us_messages_drop_columns(manager).await?;
        Ok(())
    }
}

impl Migration {
    async fn alter_contact_us_messages_add_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(ContactUsMessages::Table)
                .add_column(ColumnDef::new(ContactUsMessages::CompanyAddress)
                    .text()
                )
                .add_column(ColumnDef::new(ContactUsMessages::Designation)
                    .text()
                )
                .to_owned(),
        ).await?;
        Ok(())
    }
    async fn alter_contact_us_messages_drop_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(ContactUsMessages::Table)
                .drop_column(ContactUsMessages::CompanyAddress)
                .drop_column(ContactUsMessages::Designation)
                .to_owned(),
        ).await?;
        Ok(())
    }
}
