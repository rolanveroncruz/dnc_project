use sea_orm_migration::{prelude::* };
use crate::m20251221_124454_create_table_dental_services::DentalService;
use crate::m20260126_161604_create_table_dentists::Dentist;
use crate::m20260307_083354_add_endorsement_rates_masterlists::MasterListMember;
use crate::m20260408_060317_alter_table_verification_add_tooth_service_type_and_tooth_surface::{ToothSurface, ToothServiceType};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_accomplishment_reconciliation_table(manager).await?;
        Self::alter_verification_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::reverse_alter_verification_table(manager).await?;
        Self::drop_accomplishment_reconciliation_table(manager).await?;
        Ok(())
    }
}

impl Migration {
    async fn create_accomplishment_reconciliation_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccReconciliation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AccReconciliation::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(AccReconciliation::DateCreated)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                    )
                    .col(ColumnDef::new(AccReconciliation::CreatedBy)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(AccReconciliation::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_acc_recon_table_dentist_id")
                        .from(AccReconciliation::Table, AccReconciliation::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(AccReconciliation::MemberId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_acc_recon_table_member_id")
                        .from(AccReconciliation::Table, AccReconciliation::MemberId)
                        .to(MasterListMember::Table, MasterListMember::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(AccReconciliation::DentalServiceId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_acc_recon_table_dental_service_id")
                        .from(AccReconciliation::Table, AccReconciliation::DentalServiceId)
                        .to(DentalService::Table, DentalService::Id)
                    )
                    .col(ColumnDef::new(AccReconciliation::DateServicePerformed)
                        .date()
                    )
                    .col(ColumnDef::new(AccReconciliation::ApprovedBy)
                        .string()
                    )
                    .col(ColumnDef::new(AccReconciliation::ApprovalDate)
                        .timestamp_with_time_zone()
                    )
                    .col(ColumnDef::new(AccReconciliation::ApprovalCode)
                        .string()
                    )
                    .col(ColumnDef::new(AccReconciliation::ToothId)
                        .string()
                    )
                    .col( ColumnDef::new(AccReconciliation::ToothServiceTypeId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_acc_recon_table_tooth_service_type_id")
                        .from(AccReconciliation::Table, AccReconciliation::ToothServiceTypeId)
                        .to(ToothServiceType::Table, ToothServiceType::Id)
                    )
                    .col(ColumnDef::new(AccReconciliation::ToothSurfaceId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_acc_recon_table_tooth_surface_id")
                        .from(AccReconciliation::Table, AccReconciliation::ToothSurfaceId)
                        .to(ToothSurface::Table, ToothSurface::Id)
                    )
                .to_owned(),
            ).await?;
        Ok(())
    }
    async fn drop_accomplishment_reconciliation_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AccReconciliation::Table)
                    .to_owned(),
            ).await?;
        Ok(())
    }

    async fn alter_verification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_column( ColumnDef::new(Verification::IsReconciled)
                        .boolean()
                        .default(false)
                    )
                    .add_column( ColumnDef::new(Verification::ReconciledBy)
                        .string()
                    )
                    .add_column( ColumnDef::new(Verification::ReconciliationDate)
                        .timestamp_with_time_zone()
                    ).to_owned(),
            ).await?;
        Ok(())
    }
    async fn reverse_alter_verification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_column(Verification::IsReconciled)
                    .drop_column(Verification::ReconciledBy)
                    .drop_column(Verification::ReconciliationDate)
                .to_owned(),
            ).await?;
        Ok(())
    }
}


/*
The Accomplishment Reconciliation (AccReconciliation) table is a transitionary table to allow reconciliation
between system records and dentist's accomplishment reports.
It is structurally identical to the Verification Table. Ideally, there is no need for this, as there is no way an approval code
could be released without the system releasing it. But in the unlikely chance that that happens,
we have a record of it, i.e., a dentist attempting to defraud DNC.
 */
#[derive(DeriveIden)]
pub enum AccReconciliation {
    Table,
    Id,
    DateCreated,
    CreatedBy,
    DentistId,
    MemberId,
    DentalServiceId,
    DateServicePerformed,
    ApprovedBy,
    ApprovalDate,
    ApprovalCode,
    ToothId,
    ToothSurfaceId,
    ToothServiceTypeId,
}

/*
We are adding columns to Verification to allow verifications to be "reconciled" against Dentists' Accomplishment Report.
We put these in the Verifications table instead of the AccReconciliation table, so that if the AccReconciliation table is
always empty, we know that we are capturing all data properly.
 */
#[derive(DeriveIden)]
pub enum Verification {
    Table,
    IsReconciled,
    ReconciledBy,
    ReconciliationDate,
}


