use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_unified_view(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_unified_view(manager).await?;
        Ok(())
    }
}

impl Migration{
    pub async fn create_unified_view(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
            CREATE VIEW unified_approved AS
                SELECT
                    id,
                    date_created,
                    dentist_id,
                    dentist_name,
                    company_id,
                    company_name,
                    member_id,
                    member_account_number,
                    member_name,
                    dental_service_name,
                    date_service_performed,
                    tooth
                FROM verification_with_details
            UNION ALL
                SELECT
                    id,
                    date_created,
                    dentist_id,
                    dentist_name,
                    company_id,
                    company_name,
                    member_id,
                    member_account_number,
                    member_name,
                    dental_service_name,
                    date_service_performed,
                    tooth
                FROM acc_recon_with_details;
            "#).await?;
        Ok(())
    }
    pub async fn drop_unified_view(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"DROP VIEW unified_approved"#
            ).await?;
        Ok(())
    }
}