use sea_orm_migration::{prelude::*, };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(DentistContract::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistContract::Id)
                        .integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistContract::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistContract::Description)
                        .string()
                        .default("")
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistContract::Active)
                        .boolean()
                        .default(true)
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistContract::LastModifiedBy)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistContract::LastModifiedOn)
                             .timestamp_with_time_zone()
                             .not_null()
                             .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DentistContract::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum DentistContract{
    Table,
    Id,
    Name,
    Description,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
