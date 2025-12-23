use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // 1. Create the Role table
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(ColumnDef::new(Role::Name)
                        .string()
                        .not_null()
                    )
                    .col(string(Role::Description))
                    .to_owned(),
            )
            .await?;

        // 2. Insert the Administrator Role
        Self::insert_role(manager, "Administrator", "Administrator Role").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}
impl Migration{
    async fn insert_role(manager:&SchemaManager<'_>, role_name:&str, description:&str)->Result<(), DbErr>{
        let insert = Query::insert()
            .into_table(Role::Table)
            .columns([Role::Name, Role::Description])
            .values_panic([Expr::val(role_name), Expr::val(description)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

}
#[derive(Iden)]
pub enum Role{
    Table,
    Id,
    Name,
    Description,
}
