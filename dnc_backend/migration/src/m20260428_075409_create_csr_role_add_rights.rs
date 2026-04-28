use sea_orm_migration::{prelude::* };
use crate::m20251205_075435_create_table_role::Migration as CreateTableRole;
use crate::m20251205_075445_create_table_role_permission::Migration as RolePermission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        CreateTableRole::insert_role(manager, "CSR", "Customer Service Rep").await?;
        RolePermission::insert_role_all_permissions(manager, "CSR", "verifications").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        CreateTableRole::drop_role(manager, "CSR" ).await?;
        RolePermission::del_role_all_permissions(manager, "CSR", "verifications").await?;
        Ok(())
    }
}
