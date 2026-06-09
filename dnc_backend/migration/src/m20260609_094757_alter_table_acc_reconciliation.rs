use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            ALTER TABLE acc_reconciliation
                DROP CONSTRAINT IF EXISTS fk_acc_recon_table_tooth_surface_id;

            ALTER TABLE acc_reconciliation
                DROP COLUMN IF EXISTS tooth_surface_id;

            ALTER TABLE acc_reconciliation
                ADD COLUMN tooth_surface_names TEXT;
            "#
        )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            ALTER TABLE acc_reconciliation
                DROP COLUMN IF EXISTS tooth_surface_names;

            ALTER TABLE acc_reconciliation
                ADD COLUMN tooth_surface_id INTEGER;

            ALTER TABLE acc_reconciliation
                ADD CONSTRAINT fk_acc_recon_table_tooth_surface_id
                FOREIGN KEY (tooth_surface_id)
                REFERENCES tooth_surface(id);
            "#
        )
            .await?;
        Ok(())
    }
}
