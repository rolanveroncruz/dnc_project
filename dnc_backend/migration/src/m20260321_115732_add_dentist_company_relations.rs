use sea_orm_migration::{prelude::* };

use crate::{m20260126_161604_create_table_dentists::Dentist};
use crate::m20260220_082933_create_endorsement_tables::{ EndorsementCompany};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DentistCompanyRelations::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(DentistCompanyRelations::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .col(ColumnDef::new(DentistCompanyRelations::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_dentist_company_relations_dentist_id")
                        .from (DentistCompanyRelations::Table, DentistCompanyRelations::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                    )
                    .col(ColumnDef::new(DentistCompanyRelations::CompanyId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_dentist_company_relations_company_id")
                        .from(DentistCompanyRelations::Table, DentistCompanyRelations::CompanyId)
                        .to(EndorsementCompany::Table, EndorsementCompany::Id))
                    .col(ColumnDef::new(DentistCompanyRelations::IsExclusiveToCompany)
                        .boolean()
                    )
                    .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DentistCompanyRelations::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum DentistCompanyRelations{
    Table,
    Id,
    DentistId,
    CompanyId,
    IsExclusiveToCompany,
}
