use sea_orm_migration::{prelude::*};
use crate::m20260126_063012_create_tables_dental_clinic::DentalClinic;
use crate::m20260126_161604_create_table_dentists::Dentist;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_dentist_payments_table(self, manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       Self::drop_dentist_payments_table(self, manager).await?;
       Ok(())
    }
}
impl Migration{
    async fn create_dentist_payments_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .create_table(
                Table::create()
                    .table(DentistPayments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistPayments::Id)
                        .integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistPayments::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_dentist_payments_dentist_id")
                        .from(DentistPayments::Table, DentistPayments::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                    )
                    .col(ColumnDef::new(DentistPayments::ClinicId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_dentist_payments_clinic_id")
                        .from(DentistPayments::Table, DentistPayments::ClinicId)
                        .to(DentalClinic::Table, DentalClinic::Id)
                    )
                    .col(ColumnDef::new(DentistPayments::Year)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistPayments::Month)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentistPayments::ReportName)
                        .string()
                    )
                    .col(ColumnDef::new(DentistPayments::DatePaid)
                        .timestamp_with_time_zone()
                    )
                    .col(ColumnDef::new(DentistPayments::DatePaidRecordedBy)
                        .string()
                    )
                    .to_owned()
            ).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dentist_payments_unique_dentist_year_month")
                    .table(DentistPayments::Table)
                    .col(DentistPayments::DentistId)
                    .col(DentistPayments::Year)
                    .col(DentistPayments::Month)
                    .unique()
                    .to_owned()
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
            ALTER TABLE dentist_payments
            ADD CONSTRAINT chk_dentist_payments_month
            CHECK (month BETWEEN 1 AND 12)
            "#,
            )
            .await?;

        Ok(())
    }
    async fn drop_dentist_payments_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(DentistPayments::Table).to_owned()).await?;
        Ok(())
    }
}


/*
  In this table, we record the months on which a dentist+clinic has been paid their retainer.
  For easy querying, we record the year and month to which the payment was made.
  We could have
 */
#[derive(DeriveIden)]
pub enum DentistPayments{
    Table,
    Id,
    DentistId,
    ClinicId,
    Year,       // for easy insertion and intuition
    Month,      // for easy insertion and intuition
    ReportName, // the name of the report which generated the payment.
    DatePaid,   // reserve this for future use to record when a payment was actually made.
    DatePaidRecordedBy, // user who said payment was made.
}
