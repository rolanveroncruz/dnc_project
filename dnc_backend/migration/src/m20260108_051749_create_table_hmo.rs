use sea_orm_migration::{prelude::*, };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(HMO::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(HMO::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(HMO::ShortName)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(HMO::LongName)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(HMO::Address)
                        .string()
                    )
                    .col(ColumnDef::new(HMO::TaxAccountNumber)
                    .string()
                    )
                    .col(ColumnDef::new(HMO::ContactNos)
                        .string()
                    )
                    .col(ColumnDef::new(HMO::Active)
                        .boolean()
                        .default(true)
                        .not_null()
                    )
                    .col(ColumnDef::new(HMO::LastModifiedBy)
                        .string()
                        .not_null()
                        .default("system")
                    )
                    .col(ColumnDef::new(HMO::LastModifiedOn)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await?;

        Self::insert_hmo(manager, "AFDR", "AFDR Insurance Brokers, Inc.").await?;
        Self::insert_hmo(manager, "Avega", "Avega Managed Care, Inc.").await?;
        Self::insert_hmo(manager, "Etiqa", "Etiqua Life and General Assurance Philippines, Inc.").await?;
        Self::insert_hmo(manager, "Intellicare", "Intellicare").await?;
        Self::insert_hmo(manager, "Kaiser", "Kaiser International Healthgroup, Inc.").await?;
        Self::insert_hmo(manager, "HMI", "Health Maintenance Inc.").await?;
        Self::insert_hmo(manager, "Maxicare", "MaxiCare Corporation").await?;
        Self::insert_hmo(manager, "Medicare Plus", "Medicare Plus, Inc.").await?;
        Self::insert_hmo(manager, "PhilCare", "PhilCare").await?;
        Self::insert_hmo(manager, "Responsive", "Responsive Health & Insurance Brokers").await?;
        Self::insert_hmo(manager, "Simple PPA", "Simple PPA").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HMO::Table).to_owned())
            .await
    }
}

impl Migration{
    async fn insert_hmo(manager: &SchemaManager<'_>, short_name: &str, long_name:&str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(HMO::Table)
            .columns([HMO::ShortName, HMO::LongName])
            .values_panic([short_name.into(), long_name.into()])
            .to_owned();
        manager.exec_stmt(insert).await
    }
}


#[derive(Iden)]
pub enum HMO{
    Table,
    Id,
    ShortName,
    LongName,
    Address,
    TaxAccountNumber,
    ContactNos,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
