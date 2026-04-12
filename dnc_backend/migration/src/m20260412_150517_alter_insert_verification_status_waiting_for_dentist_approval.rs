use sea_orm_migration::{prelude::* };
use crate::m20260319_052702_add_verification_tables::VerificationStatus;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::insert_into_verification_status_table(manager, 21, "Waiting for Dentist Approval").await?;
        Ok(())

    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

impl Migration{
    async fn insert_into_verification_status_table(manager: &SchemaManager<'_>, int_code:i32, name:&str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(VerificationStatus::Table)
            .columns(vec![VerificationStatus::IntCode,VerificationStatus::Name])
            .values_panic([Expr::val(int_code),Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

}