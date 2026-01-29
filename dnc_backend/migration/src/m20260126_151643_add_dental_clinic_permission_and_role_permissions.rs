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
        PermissionMigration::add_all_permissions(manager, "province").await?;
        PermissionMigration::add_all_permissions(manager, "city").await?;
        PermissionMigration::add_all_permissions(manager, "dental_clinic").await?;
        PermissionMigration::add_all_permissions(manager, "clinic_capabilities_list").await?;

        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "region").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "province").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "city").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "dental_clinic").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "clinic_capabilities_list").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "clinic_capabilities_list").await?;
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "dental_clinic").await?;
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "city").await?;
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "province").await?;
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "region").await?;

        PermissionMigration::del_all_permissions(manager, "clinic_capabilities_list").await?;
        PermissionMigration::del_all_permissions(manager, "dental_clinic").await?;
        PermissionMigration::del_all_permissions(manager, "city").await?;
        PermissionMigration::del_all_permissions(manager, "province").await?;
        PermissionMigration::del_all_permissions(manager, "region").await?;

        DataObjectMigration::delete_dataobject(manager, "clinic_capabilities_list").await?;
        DataObjectMigration::delete_dataobject(manager, "dental_clinic").await?;
        DataObjectMigration::delete_dataobject(manager, "city").await?;
        DataObjectMigration::delete_dataobject(manager, "province").await?;
        DataObjectMigration::delete_dataobject(manager, "region").await?;


        Ok(())
    }
}
