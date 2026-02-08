use sea_orm_migration::{prelude::*};
use crate::m20260108_051749_create_table_hmo::HMO;
use crate::m20260126_161604_create_table_dentists::Dentist;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(DentistHMORelations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistHMORelations::Id)
                        .integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistHMORelations::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_hmo_relations_dentist_id_foreign_key")
                        .from(DentistHMORelations::Table, DentistHMORelations::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                    )
                    .col(ColumnDef::new(DentistHMORelations::HMOId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_hmo_relations_hmo_id_foreign_key")
                        .from(DentistHMORelations::Table, DentistHMORelations::HMOId)
                        .to(HMO::Table, HMO::Id)
                    )
                    .col(ColumnDef::new(DentistHMORelations::IsExclusiveToHMO)
                        .boolean()
                        .default(true)
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DentistHMORelations::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum DentistHMORelations{
    Table,
    Id,
    DentistId,
    HMOId,
    IsExclusiveToHMO,
}
