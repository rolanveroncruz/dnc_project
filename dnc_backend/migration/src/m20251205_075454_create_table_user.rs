use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251205_075435_create_table_role::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create the User table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(ColumnDef::new(User::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Email)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Password)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::RoleId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_role_id_foreign_key")
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    ).to_owned()
            ).await?;

        // 2. Create the Admin user
        let insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Name, User::Email, User::Password, User::RoleId])
            .select_from(
                Query::select()
                    .expr(Expr::val("Admin"))
                    .expr(Expr::val("admin@dnc.com.ph"))
                    .expr(Expr::val("password"))

                    .expr(
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(
                                Query::select()
                                    .column(Role::Id)
                                    .from(Role::Table)
                                    .and_where(Expr::col(Role::Name).eq("Administrator"))
                                    .limit(1)
                                    .to_owned()
                                    .into()
                            )
                        )
                    )
                    .to_owned()
            ).map_err(|e|DbErr::Custom(format!("{:?}", e)))?
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User{
    Table,
    Id,
    Name,
    Email,
    Password,
    RoleId,
}
