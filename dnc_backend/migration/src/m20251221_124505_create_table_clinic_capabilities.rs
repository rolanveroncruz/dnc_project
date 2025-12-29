use sea_orm_migration::{prelude::*, };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClinicCapability::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ClinicCapability::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(ClinicCapability::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(ClinicCapability::Active)
                        .boolean()
                        .default(true)
                        .not_null()
                    )
                    .col(ColumnDef::new(ClinicCapability::LastModifiedBy)
                        .string()
                        .not_null()
                        .default("system")
                    )
                    .col(ColumnDef::new(ClinicCapability::LastModifiedOn)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .to_owned()
            ).await?;

        Self::insert_clinic_capabilities(manager, "Dental Services").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClinicCapability::Table).to_owned())
            .await?;
        Ok(())
    }
}

impl Migration {
    async fn insert_clinic_capabilities(manager: &SchemaManager<'_>, capability: &str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(ClinicCapability::Table)
            .columns([ClinicCapability::Name])
            .values_panic([capability.into()])
            .to_owned();
        manager.exec_stmt(insert).await
    }
}
#[derive(Iden)]
pub enum ClinicCapability{
    Table,
    Id,
    Name,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
