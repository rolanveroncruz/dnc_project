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
        let insert = Query::insert()
            .into_table(Role::Table)
            .columns([Role::Name, Role::Description])
            .values_panic(["Administrator".into(), "Admin Role".into()])
            .to_owned();

        manager.execute(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum Role{
    Table,
    Id,
    Name,
    Description,
}
