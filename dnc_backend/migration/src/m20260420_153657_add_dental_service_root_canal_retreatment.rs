use sea_orm_migration::{prelude::* };
use crate::m20251221_124454_create_table_dental_services;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        m20251221_124454_create_table_dental_services::Migration::create_dental_service(manager, "Root Canal Treatment", "High-End",false, false, "system").await?;
        Ok(())
    }


    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
