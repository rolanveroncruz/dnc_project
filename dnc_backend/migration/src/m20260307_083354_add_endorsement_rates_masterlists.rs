use sea_orm_migration::{prelude::*};
use crate::m20251221_124454_create_table_dental_services::DentalService;
use crate::m20260220_082933_create_endorsement_tables::Endorsement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        Self::create_master_list_table(manager).await?;
        Self::create_master_list_member_table(manager).await?;
        Self::create_endorsement_rates_table(manager).await?;
        Self::create_endorsement_counts_table(manager).await?;
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_endorsement_counts_table(manager).await?;
        Self::drop_endorsement_rates_table(manager).await?;
        Self::drop_master_list_member_table(manager).await?;
        Self::drop_master_list_table(manager).await?;
        Ok(())
    }
}
impl Migration {
    pub async fn create_master_list_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MasterList::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MasterList::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MasterList::FileName).string().not_null())
                    .col(ColumnDef::new(MasterList::EndorsementId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_master_list_endorsement_id")
                            .from(MasterList::Table, MasterList::EndorsementId)
                            .to(Endorsement::Table, Endorsement::Id),
                    )
                    .col(ColumnDef::new(MasterList::UploadedBy).string())
                    .col(
                        ColumnDef::new(MasterList::UploadDate)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    pub async fn drop_master_list_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MasterList::Table).to_owned())
            .await?;
        Ok(())
    }


    pub async fn create_master_list_member_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(Table::create()
                .table(MasterListMember::Table)
                .if_not_exists()
                .col(ColumnDef::new(MasterListMember::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
                )
                // MasterList Member is always associated with an endorsement.
                .col(ColumnDef::new(MasterListMember::EndorsementId)
                    .integer()
                    .not_null()
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_master_list_member_endorsement_id")
                        .from(MasterListMember::Table, MasterList::EndorsementId)
                        .to(Endorsement::Table, Endorsement::Id),
                )
                // Optionally, a MasterListMember could be associated with a MasterList.
                .col(ColumnDef::new(MasterListMember::MasterListId)
                    .integer()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_master_list_id")
                    .from(MasterListMember::Table, MasterListMember::MasterListId)
                    .to(MasterList::Table, MasterList::Id)
                )
                .col(ColumnDef::new(MasterListMember::AccountNumber)
                    .string()
                    .not_null()
                )
                .col(ColumnDef::new(MasterListMember::LastName)
                    .string()
                    .not_null()
                )
                .col(ColumnDef::new(MasterListMember::FirstName)
                    .string()
                    .not_null()
                )
                .col(ColumnDef::new(MasterListMember::MiddleName)
                    .string()
                    .not_null()
                )
                .col(ColumnDef::new(MasterListMember::EmailAddress)
                .string()
                )
                .col(ColumnDef::new(MasterListMember::MobileNumber)
                .string()
                )
                .col(ColumnDef::new(MasterListMember::BirthDate)
                    .date()
                )
                .col(ColumnDef::new(MasterListMember::IsActive)
                    .boolean()
                    .default(true)
                    .not_null()
                )
                    .to_owned()
            ).await?;

        Ok(())
    }
    pub async fn drop_master_list_member_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MasterListMember::Table).to_owned())
            .await?;
        Ok(())
    }

    pub async fn create_endorsement_rates_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(EndorsementRates::Table)
                .if_not_exists()
                .col(ColumnDef::new(EndorsementRates::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
                )
                .col(ColumnDef::new(EndorsementRates::EndorsementId)
                    .integer()
                    .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_endorsement_id")
                    .from(EndorsementRates::Table, EndorsementRates::EndorsementId)
                    .to(Endorsement::Table, Endorsement::Id)
                )
                .col(ColumnDef::new(EndorsementRates::DentalServicesId)
                    .integer()
                    .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_dental_services_id")
                    .from(EndorsementRates::Table, EndorsementRates::DentalServicesId)
                    .to(DentalService::Table, DentalService::Id)
                )
                .col(ColumnDef::new(EndorsementRates::Rate)
                    .decimal()
                    .not_null()
                )
                .to_owned()
        ).await?;
        Ok(())

    }
    pub async fn drop_endorsement_rates_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(EndorsementRates::Table).to_owned()).await?;
        Ok(())
    }

    pub async fn create_endorsement_counts_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table( Table::create()
                .table(EndorsementCounts::Table)
                    .if_not_exists()
                .col(ColumnDef::new(EndorsementCounts::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
                )
                .col(ColumnDef::new(EndorsementCounts::EndorsementId)
                    .integer()
                    .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_endorsement_count_endorsement_id")
                    .from(EndorsementCounts::Table, EndorsementCounts::EndorsementId)
                    .to(Endorsement::Table, Endorsement::Id)
                )
                .col(ColumnDef::new(EndorsementCounts::DentalServicesId)
                    .integer()
                    .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .name("fk_endorsement_count_dental_services_id")
                    .from(EndorsementCounts::Table, EndorsementCounts::DentalServicesId)
                    .to(DentalService::Table, DentalService::Id)
                )
                .col(ColumnDef::new(EndorsementCounts::Count)
                    .integer()
                    .not_null()
                )
                .to_owned()).await?;
        Ok(())

    }

    pub async fn drop_endorsement_counts_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(EndorsementCounts::Table).to_owned()).await?;
        Ok(())
    }

}
/*
The MasterList represents every uploaded file.
We only keep it so we can track who uploaded it, when it was uploaded,
and where a member came from.
*/
#[derive(Iden)]
pub enum MasterList {
    Table,
    Id,
    FileName,
    EndorsementId,
    UploadedBy,
    UploadDate,
}
/*
TheMasterListMember lists all members of that MasterList.
 */
#[derive(Iden)]
pub enum MasterListMember {
    Table,
    EndorsementId,
    Id,
    MasterListId,
    AccountNumber,
    LastName,
    FirstName,
    MiddleName,
    EmailAddress,
    BirthDate,
    MobileNumber,
    IsActive,
}
#[derive(Iden)]
pub enum EndorsementRates {
    Table,
    Id,
    EndorsementId,
    DentalServicesId,
    Rate,
}

#[derive(Iden)]
pub enum EndorsementCounts{
    Table,
    Id,
    EndorsementId,
    DentalServicesId,
    Count,
}
