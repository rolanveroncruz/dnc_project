use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
             UPDATE dental_service
            SET record_tooth = TRUE
            WHERE name IN (
                'Temporary Fillings',
                'Permanent Fillings (per Tooth)',
                'Simple Tooth Extraction',
                'Recementation of Jacket Crown'
            );
            "#
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
             UPDATE dental_service
            SET record_tooth = FALSE
            WHERE name IN (
                'Temporary Fillings',
                'Permanent Fillings (per Tooth)',
                'Simple Tooth Extraction',
                'Recementation of Jacket Crown'
            );
            "#
        ).await?;
        Ok(())
    }
}
