use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

const OLD_INDEX_NAME: &str = "idx_dentist_payments_unique_dentist_year_month";
const NEW_INDEX_NAME: &str = "idx_dentist_payments_unique_dentist_clinic_year_month";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the old unique index:
        // UNIQUE (dentist_id, year, month)
        manager
            .drop_index(
                Index::drop()
                    .name(OLD_INDEX_NAME)
                    .table(DentistPayments::Table)
                    .to_owned(),
            )
            .await?;

        // Create the new unique index:
        // UNIQUE (dentist_id, clinic_id, year, month)
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name(NEW_INDEX_NAME)
                    .table(DentistPayments::Table)
                    .col(DentistPayments::DentistId)
                    .col(DentistPayments::ClinicId)
                    .col(DentistPayments::Year)
                    .col(DentistPayments::Month)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the new unique index.
        manager
            .drop_index(
                Index::drop()
                    .name(NEW_INDEX_NAME)
                    .table(DentistPayments::Table)
                    .to_owned(),
            )
            .await?;

        // Restore the old unique index.
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name(OLD_INDEX_NAME)
                    .table(DentistPayments::Table)
                    .col(DentistPayments::DentistId)
                    .col(DentistPayments::Year)
                    .col(DentistPayments::Month)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum DentistPayments{
    Table,
    DentistId,
    ClinicId,
    Year,       // for easy insertion and intuition
    Month,      // for easy insertion and intuition
}
