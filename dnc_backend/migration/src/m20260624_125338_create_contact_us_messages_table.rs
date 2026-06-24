use sea_orm_migration::{prelude::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ContactUsMessages {
    Table,
    Id,
    DateSubmitted,
    PersonType,
    Name,
    CardNumber,
    CompanyAndHmo,
    ContactNumbers,
    Message,
    Status,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactUsMessages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactUsMessages::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactUsMessages::DateSubmitted)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(ContactUsMessages::PersonType).text().not_null())
                    .col(ColumnDef::new(ContactUsMessages::Name).text().not_null())
                    .col(ColumnDef::new(ContactUsMessages::CardNumber).text().null())
                    .col(ColumnDef::new(ContactUsMessages::CompanyAndHmo).text().null())
                    .col(ColumnDef::new(ContactUsMessages::ContactNumbers).text().not_null())
                    .col(ColumnDef::new(ContactUsMessages::Message).text().not_null())
                    .col(
                        ColumnDef::new(ContactUsMessages::Status)
                            .text()
                            .not_null()
                            .default("new"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ContactUsMessages::Table)
                    .to_owned(),
            )
            .await
    }
}
