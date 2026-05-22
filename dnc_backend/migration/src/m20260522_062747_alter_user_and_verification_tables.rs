use sea_orm_migration::{prelude::*};
use crate::m20251205_075454_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_user_table_email_unique(manager).await?;
        Self::create_verification_user_activities_indexes(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_user_table_drop_email_unique(manager).await?;
        Self::drop_verification_user_activities_indexes(manager).await?;
        Ok(())
    }
}
impl Migration{
    pub async fn alter_user_table_email_unique(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("user_email_unique")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
    pub async fn alter_user_table_drop_email_unique(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("user_email_unique")
                    .table(User::Table)
                    .to_owned(),
            ).await?;
        Ok(())
    }

    pub async fn create_verification_user_activities_indexes(manager: &SchemaManager<'_>)->Result<(), DbErr>{
        manager
            .create_index(
                Index::create()
                    .name("verification_created_by_date_created_idx")
                    .table(Verification::Table)
                    .col(Verification::CreatedBy)
                    .col(Verification::DateCreated)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            CREATE INDEX verification_approved_by_approval_date_idx
            ON verification (approved_by, approval_date)
            WHERE approved_by IS NOT NULL
            "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            CREATE INDEX verification_reconciled_by_reconciliation_date_idx
            ON verification (reconciled_by, reconciliation_date)
            WHERE reconciled_by IS NOT NULL
            "#,
            )
            .await?;

        Ok(())
    }

    pub async fn drop_verification_user_activities_indexes(manager: &SchemaManager<'_>)->Result<(), DbErr>{
        manager
            .drop_index(
                Index::drop()
                    .name("verification_reconciled_by_reconciliation_date_idx")
                    .table(Verification::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("verification_approved_by_approval_date_idx")
                    .table(Verification::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("verification_created_by_date_created_idx")
                    .table(Verification::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
#[derive(DeriveIden)]
enum Verification {
    Table,
    CreatedBy,
    DateCreated,
}