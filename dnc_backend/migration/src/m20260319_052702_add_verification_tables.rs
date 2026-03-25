use sea_orm_migration::{prelude::*};
use crate::m20251221_124454_create_table_dental_services::DentalService;
use crate::m20260126_161604_create_table_dentists::Dentist;
use crate::m20260307_083354_add_endorsement_rates_masterlists::MasterListMember;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_verification_status_table(manager).await?;
        Self::create_verification_table(manager).await?;
        Self::create_high_end_verification_information_table(manager).await?;
        Self::create_high_end_files_table(manager).await?;
        Self::seed_verification_status_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_verification_status_table(manager).await?;
        Self::drop_verification_table(manager).await?;
        Self::drop_high_end_verification_information_table(manager).await?;
        Self::drop_high_end_files_table(manager).await?;
        Ok(())
    }
}
impl Migration {
    /* Verification Status */
    async fn create_verification_status_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VerificationStatus::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(VerificationStatus::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(VerificationStatus::Name)
                        .string()
                        .not_null()
                    )
                .to_owned(),
            ).await?;
        Ok(())
    }
    async fn drop_verification_status_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(VerificationStatus::Table).to_owned()).await?;
        Ok(())
    }
    async fn insert_into_verification_status_table(manager: &SchemaManager<'_>, name:&str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(VerificationStatus::Table)
            .columns(vec![VerificationStatus::Name])
            .values_panic([Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn seed_verification_status_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        Self::insert_into_verification_status_table(manager, "Waiting").await?;
        Self::insert_into_verification_status_table(manager, "Pending Approval Code").await?;
        Self::insert_into_verification_status_table(manager, "Approval Code Released").await?;
        Ok(())
    }


    /* Verification  */
    async fn create_verification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(Verification::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Verification::DateCreated)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                    )
                    .col(ColumnDef::new(Verification::CreatedBy)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Verification::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_verification_table_dentist_id")
                        .from(Verification::Table, Verification::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(Verification::MemberId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_verification_table_member_id")
                        .from(Verification::Table, Verification::MemberId)
                        .to(MasterListMember::Table, MasterListMember::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(Verification::DentalServiceId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_verification_table_dental_service_id")
                        .from(Verification::Table, Verification::DentalServiceId)
                        .to(DentalService::Table, DentalService::Id)
                    )
                    .col(ColumnDef::new(Verification::DateServicePerformed)
                        .date()
                    )
                    .col(ColumnDef::new(Verification::StatusId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_verification_table_status_id")
                        .from(Verification::Table, Verification::StatusId)
                        .to(VerificationStatus::Table, VerificationStatus::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(Verification::ApprovedBy)
                        .string()
                    )
                    .col(ColumnDef::new(Verification::ApprovalDate)
                        .timestamp_with_time_zone()
                    )
                    .col(ColumnDef::new(Verification::ApprovalCode)
                        .string()
                    )
                    .to_owned()).await?;
        Ok(())
    }
    async fn drop_verification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Verification::Table).to_owned()).await?;
        Ok(())
    }

    /* HighEndVerificationInformation */
    async fn create_high_end_verification_information_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HighEndVerificationInformation::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(HighEndVerificationInformation::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(HighEndVerificationInformation::VerificationId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_high_end_verification_information_verification_id")
                        .from(HighEndVerificationInformation::Table, HighEndVerificationInformation::VerificationId)
                        .to(Verification::Table, Verification::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
        //            .col(ColumnDef::new(HighEndVerificationInformation::XRayFilename).string().not_null())
                    .col(ColumnDef::new(HighEndVerificationInformation::ApprovedBy).string())
                    .col(ColumnDef::new(HighEndVerificationInformation::ApprovedCost).decimal())
                    .col(ColumnDef::new(HighEndVerificationInformation::ApprovalDate).timestamp())
                    .to_owned()).await?;
        Ok(())
    }
    async fn drop_high_end_verification_information_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(HighEndVerificationInformation::Table).to_owned()).await?;
        Ok(())
    }
    /* HighEndFiles */
    async fn create_high_end_files_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HighEndFiles::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(HighEndFiles::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(HighEndFiles::VerificationId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_high_end_files_table_verification_id")
                        .from(HighEndFiles::Table, HighEndFiles::Id)
                        .to(Verification::Table, Verification::Id)
                    )
                    .col(ColumnDef::new(HighEndFiles::Filename)
                        .string()
                        .not_null()
                    )
                    .to_owned()).await?;
        Ok(())

    }
    async fn drop_high_end_files_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(HighEndFiles::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum VerificationStatus{
    Table,
    Id,
    Name
}
#[derive(Iden)]
pub enum Verification{
    Table,
    Id,
    DateCreated,
    CreatedBy,
    DentistId,
    MemberId,
    DentalServiceId,
    DateServicePerformed,
    StatusId,
    ApprovalCode,
    ApprovalDate,
    ApprovedBy,
}
#[derive(Iden)]
pub enum HighEndVerificationInformation{
    Table,
    Id,
    VerificationId,
    ApprovalDate,
    ApprovedBy,
    ApprovedCost,
}
#[derive(Iden)]
pub enum HighEndFiles{
    Table,
    Id,
    VerificationId,
    Filename
}
