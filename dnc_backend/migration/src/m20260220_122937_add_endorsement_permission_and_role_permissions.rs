use sea_orm_migration::{prelude::* };

use crate::m20251205_063628_create_table_dataobject::Migration as DataObjectMigration;
use  crate::m20251205_075427_create_table_permission::Migration as PermissionMigration;
use  crate::m20251205_075445_create_table_role_permission::Migration as RolePermissionMigration;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        DataObjectMigration::add_dataobject(manager, "endorsements", "Endorsement Data Object").await?;
        PermissionMigration::add_all_permissions(manager, "endorsements").await?;
        RolePermissionMigration::insert_role_all_permissions(manager, "Administrator", "endorsements").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        RolePermissionMigration::del_role_all_permissions(manager, "Administrator", "endorsements").await?;
        PermissionMigration::del_all_permissions(manager, "endorsements").await?;
        DataObjectMigration::delete_dataobject(manager, "endorsements").await?;
        Ok(())
    }
}
