use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use crate::m20251205_063628_create_table_dataobject::Migration as DataObjectMigration;
use  crate::m20251205_075427_create_table_permission::Migration as PermissionMigration;
use  crate::m20251205_075445_create_table_role_permission::Migration as RolePermissionMigration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        DataObjectMigration::add_dataobject(manager, "dentist_contract", "dentist_contract Data Object").await?;
        PermissionMigration::add_all_permissions(manager, "dentist_contract").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "dentist_contract").await?;
        Ok(())

    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
