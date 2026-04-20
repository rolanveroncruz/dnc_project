use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::update_tooth_service_label(manager, "Root Cleaning", "Root Fragment").await?;
        Self::update_tooth_service_label(manager, "Retreatment", "Backjob").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::update_tooth_service_label(manager, "Backjob", "Retreatment").await?;
        Self::update_tooth_service_label(manager, "Root Fragment", "Root Cleaning").await?;
        Ok(())
    }
}

impl Migration {
    async fn update_tooth_service_label(manager: &SchemaManager<'_>, from_label: &str, to_label: &str) -> Result<(), DbErr> {
        let update = Query::update()
            .table(ToothServiceType::Table)
            .value(ToothServiceType::Name, Expr::value(to_label))
            .and_where(Expr::col(ToothServiceType::Name).eq(from_label))
            .to_owned();
        manager.exec_stmt(update).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum ToothServiceType {
    Table,
    Name
}
