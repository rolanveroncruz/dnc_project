use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_app_config_table(manager).await?;
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_app_config_table(manager).await?;
        Ok(())
    }
}
impl Migration {
    async fn create_app_config_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppConfig::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppConfig::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(AppConfig::Key)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(ColumnDef::new(AppConfig::Value)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(AppConfig::ValueType)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(AppConfig::Description)
                        .string()
                    )
                    .col(ColumnDef::new(AppConfig::DateCreated)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                    )
                    .check(
                        Expr::col(AppConfig::ValueType).is_in([
                            "string",
                            "integer",
                            "decimal",
                            "boolean",
                            "date",
                            "datetime",
                            "time",
                            "json",
                        ])
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    async fn drop_app_config_table(manager: &SchemaManager<'_>)-> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(AppConfig::Table).to_owned()).await?;
        Ok(())
    }

    pub async fn insert_key_value_pair(manager: &SchemaManager<'_>, key: &str, value: &str, value_type: &str, description: &str)-> Result<(), DbErr>{
        let insert = Query::insert()
            .into_table(AppConfig::Table)
            .columns([AppConfig::Key, AppConfig::Value, AppConfig::ValueType, AppConfig::Description])
            .values_panic([Expr::val(key), Expr::val(value), Expr::val(value_type), Expr::val(description)])
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())
    }
    pub async fn delete_key_value_pair(manager: &SchemaManager<'_>, key: &str)->Result<(), DbErr>{
        // 1. Create Delete Statement
        let delete = Query::delete()
            .from_table(AppConfig::Table)
            .and_where(Expr::col(AppConfig::Key).eq(key))
            .to_owned();

        // 2. Force Postgres formatting
        let sql = delete.to_string(PostgresQueryBuilder);

        // 3. Execute
        manager
            .get_connection()
            .execute_unprepared(&sql)
            .await?;

        Ok(())
    }
}
#[derive(DeriveIden)]
pub enum AppConfig{
    Table,
    Id,
    Key,
    Value,
    ValueType,
    Description,
    DateCreated,
}
