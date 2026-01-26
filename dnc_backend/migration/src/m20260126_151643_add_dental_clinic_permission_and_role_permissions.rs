use sea_orm_migration::{prelude::* };

use crate::m20251205_063628_create_table_dataobject::Migration as DataObjectMigration;
use  crate::m20251205_075427_create_table_permission::Migration as PermissionMigration;
use  crate::m20251205_075445_create_table_role_permission::Migration as RolePermissionMigration;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        DataObjectMigration::add_dataobject(manager, "region", "region Data Object").await?;
        DataObjectMigration::add_dataobject(manager, "state", "state Data Object").await?;
        DataObjectMigration::add_dataobject(manager, "city", "city Data Object").await?;
        DataObjectMigration::add_dataobject(manager, "dental_clinic", "dental_clinic Data Object").await?;
        DataObjectMigration::add_dataobject(manager, "clinic_capabilities_list", "clinic_capabilities_list Data Object").await?;

        PermissionMigration::add_all_permissions(manager, "region").await?;
        PermissionMigration::add_all_permissions(manager, "state").await?;
        PermissionMigration::add_all_permissions(manager, "city").await?;
        PermissionMigration::add_all_permissions(manager, "dental_clinic").await?;
        PermissionMigration::add_all_permissions(manager, "clinic_capabilities_list").await?;

        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "region").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "state").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "city").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "dental_clinic").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "clinic_capabilities_list").await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
