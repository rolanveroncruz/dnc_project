use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("high_end_verification_information_unique_verification_id")
                    .table(HighEndVerificationInformation::Table)
                    .col(HighEndVerificationInformation::VerificationId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("high_end_verification_information_unique_verification_id")
                    .table(HighEndVerificationInformation::Table)
                    .to_owned(),
            )
            .await
    }
}
#[derive(DeriveIden)]
enum HighEndVerificationInformation {
    Table,
    VerificationId,
}
