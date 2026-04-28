use sea_orm_migration::{prelude::*};

use crate::m20251205_075435_create_table_role::Migration as RoleTable;
use crate::m20251205_075445_create_table_role_permission::Migration as RolePermissionTable;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        RoleTable::insert_role(manager, "Accounting", "Billing and Accounting").await?;
        RolePermissionTable::insert_role_all_permissions(manager, "Accounting", "acc_reconciliation").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        RolePermissionTable::del_role_all_permissions(manager, "Accounting", "acc_reconciliation").await?;
        RoleTable::drop_role(manager, "Accounting").await?;
        Ok(())
    }
}
