use sea_orm_migration::{prelude::* };

use crate::m20251205_063628_create_table_dataobject::Migration as DataObjectMigration;
use  crate::m20251205_075427_create_table_permission::Migration as PermissionMigration;
use  crate::m20251205_075445_create_table_role_permission::Migration as RolePermissionMigration;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        DataObjectMigration::add_dataobject(manager, "endorsement_rates", "Endorsement Rates Master List").await?;
        PermissionMigration::add_all_permissions(manager, "endorsement_rates").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "endorsement_rates").await?;
        DataObjectMigration::add_dataobject(manager, "endorsement_counts", "Endorsement Rates Master List").await?;
        PermissionMigration::add_all_permissions(manager, "endorsement_counts").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "endorsement_counts").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "endorsement_counts").await?;
        PermissionMigration::del_all_permissions(manager, "endorsement_counts").await?;
        DataObjectMigration::delete_dataobject(manager, "endorsement_counts").await?;
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "endorsement_counts").await?;
        PermissionMigration::del_all_permissions(manager, "endorsement_counts").await?;
        DataObjectMigration::delete_dataobject(manager, "endorsement_counts").await?;

        Ok(())

    }
}
