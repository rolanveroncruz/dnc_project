use sea_orm_migration::{prelude::*};
use crate::m20260319_052702_add_verification_tables::VerificationStatus;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::insert_into_verification_status_table(manager, 3, "Dentist-quoted; waiting for approval code").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       Self::delete_from_verification_status_table(manager, 3, "Dentist-quoted; waiting for approval code").await?;
        Ok(())
    }
}
impl Migration {

    async fn insert_into_verification_status_table(manager: &SchemaManager<'_>, int_code:i32, name:&str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(VerificationStatus::Table)
            .columns(vec![VerificationStatus::IntCode,VerificationStatus::Name])
            .values_panic([Expr::val(int_code),Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }
    async fn delete_from_verification_status_table(manager: &SchemaManager<'_>, int_code: i32, name:&str) -> Result<(), DbErr> {
        let delete= Query::delete()
            .from_table(VerificationStatus::Table)
            .and_where(Expr::col(VerificationStatus::IntCode).eq(int_code))
            .and_where(Expr::col(VerificationStatus::Name).eq(name))
            .to_owned();
        manager.exec_stmt(delete).await?;
        Ok(())

    }

}
