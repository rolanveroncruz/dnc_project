use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .add_column(ColumnDef::new(AccReconciliation::MemberName)
                        .boolean()
                        .default(false)
                        .not_null()
                    ).to_owned(),
            ).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .drop_column(AccReconciliation::MemberId)
                    .to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .add_column(ColumnDef::new(AccReconciliation::MemberId)
                        .boolean()
                        .default(false)
                        .not_null()
                    ).to_owned(),
            ).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .drop_column(AccReconciliation::MemberName)
                    .to_owned(),
            ).await?;
        Ok(())
    }
}

/*
In the AccReconciliation table, instead of the member_id, we'll request for the member_name. This gives as flexibility.
The user could either choose a member_name from the existing members or create a new member.
The purpose of this table is to accept evidence of a dentist attempting something for investigations later.

 */
#[derive(DeriveIden)]
pub enum AccReconciliation {
    Table,
    MemberId,
    MemberName,
}
