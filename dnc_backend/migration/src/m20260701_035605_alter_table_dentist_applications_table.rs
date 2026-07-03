use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum DentistApplications {
    Table,
    ClinicOwnershipType,
    RegistrationDocFilePath,
    SupportingDocsFilePath1,
    HMOAffiliations,
    ClinicAddress,

}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_table_dentist_applications_add_ownership_type(manager).await?;
        Self::alter_table_dentist_applications_add_other_columns(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::alter_table_dentist_applications_drop_ownership_type(manager).await?;
        Self::alter_table_dentist_applications_drop_other_columns(manager).await?;

        Ok(())
    }
}

impl Migration {
    async fn alter_table_dentist_applications_add_ownership_type(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DentistApplications::Table)
                    .add_column(ColumnDef::new(DentistApplications::ClinicOwnershipType)
                        .text()
                        .default("single_proprietorship".to_owned())
                    )
                    .to_owned()
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE dentist_applications
                ADD CONSTRAINT dentist_applications_clinic_ownership_type_allowed
                CHECK (
                    clinic_ownership_type IN (
                        'single_proprietorship',
                        'company',
                        'corporation'
                    )
                )
                "#,
            )
            .await?;

        Ok(())
    }
    async fn alter_table_dentist_applications_drop_ownership_type(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE dentist_applications
                DROP CONSTRAINT IF EXISTS dentist_applications_clinic_ownership_type_allowed
                "#,
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(DentistApplications::Table)
                    .drop_column(DentistApplications::ClinicOwnershipType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn alter_table_dentist_applications_add_other_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DentistApplications::Table)
                    .add_column(ColumnDef::new(DentistApplications::RegistrationDocFilePath)
                        .text()
                        // allow null
                    )
                    .add_column(ColumnDef::new(DentistApplications::SupportingDocsFilePath1)
                        .text()
                        // allow null
                    )
                    .add_column(ColumnDef::new(DentistApplications::HMOAffiliations)
                        .text()
                    )
                    .add_column(ColumnDef::new(DentistApplications::ClinicAddress)
                        .text()
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    async fn alter_table_dentist_applications_drop_other_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DentistApplications::Table)
                    .drop_column(DentistApplications::ClinicAddress)
                    .drop_column(DentistApplications::HMOAffiliations)
                    .drop_column(DentistApplications::SupportingDocsFilePath1)
                    .drop_column(DentistApplications::RegistrationDocFilePath)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}