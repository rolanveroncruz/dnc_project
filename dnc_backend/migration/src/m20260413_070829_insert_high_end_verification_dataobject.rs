use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        crate::m20251205_063628_create_table_dataobject::Migration::add_dataobject(manager, "high_end_verification_information", "High End Verification Dentist Approval").await?;
        crate::m20251205_075427_create_table_permission::Migration::add_all_permissions(manager, "high_end_verification_information").await?;
        crate::m20251205_075445_create_table_role_permission::Migration::insert_role_all_permissions(manager, "Administrator", "high_end_verification_information").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::m20251205_075445_create_table_role_permission::Migration::del_role_all_permissions(manager, "Administrator", "high_end_verification_information").await?;
        crate::m20251205_075427_create_table_permission::Migration::del_all_permissions(manager, "high_end_verification_information").await?;
        crate::m20251205_063628_create_table_dataobject::Migration::delete_dataobject(manager, "high_end_verification_information").await?;
        Ok(())
    }
}
