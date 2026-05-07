use sea_orm_migration::{prelude::*};
use crate::m20260507_045752_create_app_config_table::Migration as AppConfig;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        AppConfig::insert_key_value_pair(
            manager,
            "hmo_billing_day",
            "10",
            "integer",
            "Day of the month HMO billing is generated"
        ).await?;

        AppConfig::insert_key_value_pair(
            manager,
            "dentist_claims_day",
            "10",
            "integer",
            "Day of the month Dentist Claims Report is generated").await?;

        AppConfig::insert_key_value_pair(
            manager,
            "dentist_retainers_day",
            "10",
            "integer",
            "Day of the month Dentist Retainers Report is generated").await?;
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        AppConfig::delete_key_value_pair(manager, "hmo_billing_day").await?;
        AppConfig::delete_key_value_pair(manager, "dentist_claims_day").await?;
        AppConfig::delete_key_value_pair(manager, "dentist_retainers_day").await?;
        Ok(())
    }
}
