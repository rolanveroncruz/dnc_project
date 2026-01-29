use sea_orm_migration::{prelude::* };
use sea_orm_migration::sea_query::{Expr, Query, SimpleExpr};
use crate::m20251205_075435_create_table_role::{Role};
use crate::m20251205_063628_create_table_dataobject::{DataObject};
use crate::m20251205_075427_create_table_permission::{Permission, PermissionAction};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // 1. Create Table
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermission::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key())
                    .col(ColumnDef::new(RolePermission::RoleId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("role_permission_role_id_foreign_key")
                        .from(RolePermission::Table, RolePermission::RoleId)
                        .to(Role::Table, Role::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(RolePermission::PermissionId)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(RolePermission::Active)
                        .boolean()
                        .default(true)
                        .not_null()
                    )
                    .col(ColumnDef::new(RolePermission::LastModifiedBy)
                    .string()
                    .not_null()
                    .default("system")
                    )
                    .col(ColumnDef::new(RolePermission::LastModifiedOn)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .foreign_key(ForeignKey::create()
                        .name("role_permission_permission_id_foreign_key")
                        .from(RolePermission::Table, RolePermission::PermissionId)
                        .to(Permission::Table, Permission::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .to_owned(),
            )
            .await?;


        // 2. Insert Initial Data
        Self::insert_role_all_permissions(manager, "Administrator", "dental_service").await?;
        Self::insert_role_all_permissions(manager, "Administrator", "clinic_capability").await?;
        Self::insert_role_all_permissions(manager, "Administrator", "user").await?;
        Self::insert_role_all_permissions(manager, "Administrator", "role").await?;
        Self::insert_role_all_permissions(manager, "Administrator", "role_permission").await?;
        Self::insert_role_all_permissions(manager, "Administrator", "hmo").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await?;
        Ok(())
    }
}

impl Migration {
    pub async fn insert_role_all_permissions(manager: &SchemaManager<'_>, role_name: &str, resource_name:&str )->Result<(), DbErr>{
        let permissions = vec!["create", "read", "update", "delete" ];
        for permission in permissions{
            Self::insert_role_permission(manager, role_name, resource_name, permission).await?;
        }
        Ok(())
    }
    async fn insert_role_permission(manager:&SchemaManager<'_>, role_name:&str, resource_name:&str, permission_action_name:&str)->Result<(), DbErr>{
        println!("Inserting {} permission for role: {} and resource: {}", permission_action_name, role_name, resource_name);
        let insert = Query::insert()
            .into_table(RolePermission::Table)
            .columns([RolePermission::RoleId, RolePermission::PermissionId])
            .select_from(
                Query::select()
                    // --- FK 1: RoleId
                    .expr(
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(
                                Query::select()
                                    .column(Role::Id)
                                    .from(Role::Table)
                                    .and_where(Expr::col(Role::Name).eq(role_name))
                                    .limit(1)
                                    .to_owned()
                                    .into()
                            )
                        )
                    )
                    // --- FK 2: Subquery for PermissionId (Double Filter)
                    .expr(
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(
                                Query::select()
                                    .column((Permission::Table, Permission::Id))
                                    .from(Permission::Table)
                                    // 1. Join with DataObject table
                                    .join(
                                        JoinType::InnerJoin,
                                        DataObject::Table,
                                        Expr::col((Permission::Table,Permission::DataObjectId))
                                            .equals((DataObject::Table,DataObject::Id)),
                                    )
                                    // 2. Filter using the Data Object's name column
                                    .and_where( Expr::col(DataObject::Name).eq(resource_name))
                                    // 3. Filter using the PermissionAction enum
                                    .and_where(
                                        Expr::col(Permission::Action).eq(
                                            Expr::val(permission_action_name).cast_as(PermissionAction::EnumName)
                                        )
                                    )
                                    .limit(1)
                                    .to_owned()
                                    .into()
                            )
                        )
                    ).to_owned()
            )
            .map_err(|e| DbErr::Custom(e.to_string()))?
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())


    }

}
#[derive(Iden)]
enum RolePermission{
    Table,
    Id,
    RoleId,
    PermissionId,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
