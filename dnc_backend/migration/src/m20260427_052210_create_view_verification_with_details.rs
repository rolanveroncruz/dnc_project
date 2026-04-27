use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_view_verification_with_details(manager).await?;
        Self::create_view_acc_recon_with_details(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_view_verification_with_details(manager).await?;
         Self::drop_view_acc_recon_with_details(manager).await?;
        Ok(())
    }

}

impl Migration{
    async fn create_view_verification_with_details( manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE VIEW verification_with_details AS
                SELECT
                    v.id,
                    v.date_created,
                    v.dentist_id,
                    CONCAT_WS(' ', d.given_name, d.middle_name, d.last_name) AS dentist_name,
                    ec.id as company_id,
                    ec.name as company_name,
                    v.member_id,
                    mlm.account_number as member_account_number,
                    CONCAT_WS(' ', mlm.first_name, mlm.middle_name, mlm.last_name) as member_name,
                    ds.name as dental_service_name,
                    v.date_service_performed,
                    CONCAT_WS('', v.tooth_id, ts.name) as tooth
                FROM verification v
                JOIN dentist d
                    on v.dentist_id = d.id
                JOIN master_list_member mlm
                    on v.member_id = mlm.id
                JOIN endorsement e
                    on mlm.endorsement_id = e.id
                JOIN endorsement_company ec
                    on e.endorsement_company_id = ec.id
                JOIN dental_service ds
                    on v.dental_service_id = ds.id
                LEFT JOIN tooth_surface ts
                    on v.tooth_surface_id = ts.id
                WHERE v.status_id=99
                "#
            ).await?;
        Ok(())

    }

    async fn drop_view_verification_with_details( manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP VIEW verification_with_details
                "#
            ) .await?;
        Ok(())
    }

    async fn create_view_acc_recon_with_details( manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE VIEW acc_recon_with_details AS
                SELECT
                    ar.id,
                    ar.date_created,
                    ar.dentist_id,
                    CONCAT_WS(' ', d.given_name, d.middle_name, d.last_name) AS dentist_name,
                    ar.company_id,
                    ec.name as company_name,
                    ar.member_id,
                    mlm.account_number as member_account_number,
                    CONCAT_WS(' ', mlm.first_name, mlm.middle_name, mlm.last_name) as member_name,
                    ds.name as dental_service_name,
                    ar.date_service_performed,
                    CONCAT_WS('', ar.tooth_id, ts.name) as tooth
                FROM acc_reconciliation ar
                JOIN dentist d
                    on ar.dentist_id = d.id
                JOIN endorsement_company ec
                    on ar.company_id = ec.id
                JOIN master_list_member mlm
                    on ar.member_id = mlm.id
                JOIN dental_service ds
                    on ar.dental_service_id = ds.id
                LEFT JOIN tooth_surface ts
                    on ar.tooth_surface_id = ts.id

                "#
            ) .await?;
        Ok(())
    }

    async fn drop_view_acc_recon_with_details( manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP VIEW acc_recon_with_details
                "#
            ) .await?;
        Ok(())
    }
}
