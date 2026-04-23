use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, we add member_name column.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .add_column(ColumnDef::new(AccReconciliation::MemberName)
                        .string()
                    ).to_owned(),
            ).await?;

        // Then, then we make member_id nullable.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .modify_column(
                        ColumnDef::new(AccReconciliation::MemberId)
                        .integer()
                        .null()
                    )
                    .to_owned(),
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, then we make member_id not nullable.
        manager
            .alter_table(
                Table::alter()
                    .table(AccReconciliation::Table)
                    .modify_column(
                        ColumnDef::new(AccReconciliation::MemberId)
                            .integer()
                        .not_null()
                    )
                    .to_owned(),
            ).await?;
        // Then, we drop member_name column.
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
In the AccReconciliation table, we allow either a member_id or a member_name.
Since this is just to reconcile approval codes, we are not adding the member_name into the master_list_members table.
Maybe later, we can create a procedure to do that.

 */
#[derive(DeriveIden)]
pub enum AccReconciliation {
    Table,
    MemberId,
    MemberName,
}
