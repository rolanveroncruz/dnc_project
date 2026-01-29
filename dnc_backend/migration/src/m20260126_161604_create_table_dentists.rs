use sea_orm_migration::{prelude::*};
use crate::m20260119_112338_create_table_dentist_contract::DentistContract;
use crate::m20260126_063012_create_tables_dental_clinic::DentalClinic;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_dentist_status_table(manager).await?;
        Self::create_dentist_history_table(manager).await?;
        Self::create_tax_type_table(manager).await?;
        Self::create_tax_classification_table(manager).await?;
        Self::create_dentist_table(manager).await?;
        Self::create_dentist_clinic_table(manager).await?;

        Self::insert_dentist_status_seed(manager).await?;
        Self::insert_dentist_history_seed(manager).await?;
        Self::insert_tax_type_seed(manager).await?;
        Self::insert_tax_classification_seed(manager).await?;
        Self::create_dentist_permissions_and_role_permission_set(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_dentist_clinic_table(manager).await?;
        Self::drop_dentist_table(manager).await?;
        Self::drop_tax_classification_table(manager).await?;
        Self::drop_tax_type_table(manager).await?;
        Self::drop_dentist_history_table(manager).await?;
        Self::drop_dentist_status_table(manager).await?;

        Ok(())
    }


}

impl Migration{
    pub async fn create_dentist_status_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(DentistStatus::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistStatus::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(DentistStatus::Name)
                        .string()
                        .not_null()
                    ).to_owned()
            ).await?;

        Ok(())
    }
    pub async fn drop_dentist_status_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DentistStatus::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_dentist_status_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(DentistStatus::Table)
                    .columns([DentistStatus::Name])
                    .values_panic(["Accredited".into()])
                    .values_panic(["Applicant".into()])
                    .values_panic(["On Hold".into()])
                    .values_panic(["Non-accredited".into()])
                    .to_owned(),
            )
            .await?;
        Ok(())
    }


    pub async fn create_dentist_history_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DentistHistory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistHistory::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                        .col(ColumnDef::new(DentistHistory::Name)
                            .string()
                            .not_null()
                    ).to_owned()
            ).await?;
        Ok(())
    }

    pub async fn drop_dentist_history_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DentistHistory::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_dentist_history_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(DentistHistory::Table)
                    .columns([DentistHistory::Name])
                    .values_panic(["Applicant".into()])
                    .values_panic(["Requested".into()])
                    .to_owned(),
            ).await?;
        Ok(())
    }


    pub async fn create_tax_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaxType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaxType::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(TaxType::Name)
                        .string()
                        .not_null()
                    ).to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_tax_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TaxType::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_tax_type_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaxType::Table)
                    .columns([TaxType::Name])
                    .values_panic(["VAT-Reg".into()])
                    .values_panic(["Non-VAT-Reg".into()])
                    .to_owned(),
            ).await?;
        Ok(())
    }


    pub async fn create_tax_classification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaxClassification::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(TaxClassification::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(TaxClassification::Name)
                        .string()
                        .not_null()
                    ).to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_tax_classification_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TaxClassification::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_tax_classification_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaxClassification::Table)
                    .columns([TaxClassification::Name])
                    .values_panic(["Corporation".into()])
                    .values_panic(["Individual".into()])
                    .values_panic(["GPP".into()])
                    .to_owned(),
            ).await?;
        Ok(())
    }


    pub async fn create_dentist_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dentist::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(Dentist::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Dentist::LastName)
                        .string()
                        .not_null()
                        .default(" ")
                    )
                    .col(ColumnDef::new(Dentist::GivenName)
                        .string()
                        .not_null()
                        .default(" ")
                    )
                    .col(ColumnDef::new(Dentist::MiddleInitial)
                        .string()
                        .not_null()
                        .default(" ")
                    )
                    .col(ColumnDef::new(Dentist::Email)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::RetainerFee)
                        .float()
                        .not_null()
                        .default(0.0)
                    )
                    // The dentist status id is a foreign key to the dentist_status table.
                    // We allow it to be null.
                    .col(ColumnDef::new(Dentist::DentistStatusId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_dentist_status_id_foreign_key")
                        .from(Dentist::Table, Dentist::DentistStatusId)
                        .to(DentistStatus::Table, DentistStatus::Id)
                    )
                    // The dentist history id is a foreign key to the dentist_history table.
                    // We allow it to be null.
                    .col(ColumnDef::new(Dentist::DentistHistoryId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_dentist_history_id_foreign_key")
                        .from(Dentist::Table, Dentist::DentistHistoryId)
                        .to(DentistHistory::Table, DentistHistory::Id)
                    )
                    // This only has a value of history is 'Requested'
                    .col(ColumnDef::new(Dentist::DentistRequestedBy)
                        .string()
                    )
                    //The accreditation dentist contract id is a foreign key to the dentist contract table.
                    //We allow it to be null.
                    .col(ColumnDef::new(Dentist::AccreDentistContractId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_accre_dentist_contract_id_foreign_key")
                        .from(Dentist::Table, Dentist::AccreDentistContractId)
                        .to(DentistContract::Table, DentistContract::Id)
                    )
                    .col(ColumnDef::new(Dentist::AccreDocumentCode)
                        .string()
                        .default(" ")
                    )
                    .col(ColumnDef::new(Dentist::AccreditationDate)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccreContractSentDate)
                    .string()
                    )
                    .col(ColumnDef::new(Dentist::AccreContractFilePath)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccTIN)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccBankName)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccAccountName)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccAccountNumber)
                        .string()
                    )
                    .col(ColumnDef::new(Dentist::AccTaxTypeID)
                    .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_acc_tax_type_id_foreign_key")
                        .from(Dentist::Table, Dentist::AccTaxTypeID)
                        .to(TaxType::Table, TaxType::Id)
                    )
                    .col(ColumnDef::new(Dentist::AccTaxClassificationID)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_acc_tax_classification_id_foreign_key")
                        .from(Dentist::Table, Dentist::AccTaxClassificationID)
                        .to(TaxClassification::Table, TaxClassification::Id)
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_dentist_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Dentist::Table).to_owned()).await?;
        Ok(())
    }

    pub async fn create_dentist_clinic_table(manager: &SchemaManager<'_>) ->Result<(), DbErr>{
        manager
            .create_table(
                Table::create()
                    .table(DentistClinic::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentistClinic::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(DentistClinic::DentistId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_clinic_dentist_id_foreign_key")
                        .from(DentistClinic::Table, DentistClinic::DentistId)
                        .to(Dentist::Table, Dentist::Id)
                    )
                    .col(ColumnDef::new(DentistClinic::ClinicId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dentist_clinic_clinic_id_foreign_key")
                        .from(DentistClinic::Table, DentistClinic::ClinicId)
                        .to(DentalClinic::Table, DentalClinic::Id)
                    )
                    .col(ColumnDef::new(DentistClinic::Position)
                        .string()
                    )
                    .col(ColumnDef::new(DentistClinic::Schedule)
                        .string()
                    ).to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_dentist_clinic_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DentistClinic::Table).to_owned()).await?;
        Ok(())
    }


    pub async fn create_dentist_permissions_and_role_permission_set(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        crate::m20251205_063628_create_table_dataobject::Migration::add_dataobject(manager, "dentist", "dentist Data Object").await?;
        crate::m20251205_075427_create_table_permission::Migration::add_all_permissions(manager, "dentist").await?;
        crate::m20251205_075445_create_table_role_permission::Migration::insert_role_all_permissions(manager, "Administrator", "dentist").await?;

        Ok(())

    }
}

#[derive(Iden)]
pub enum DentistStatus {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
pub enum DentistHistory{
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum TaxType{
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum TaxClassification{
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Dentist {
    Table,
    Id,
    LastName,
    GivenName,
    MiddleInitial,
    Email,
    RetainerFee,
    DentistStatusId,
    DentistHistoryId,
    DentistRequestedBy,
    AccreDentistContractId,
    AccreDocumentCode,
    AccreditationDate,
    AccreContractSentDate,
    AccreContractFilePath,
    AccTIN,
    AccBankName,
    AccAccountName,
    AccAccountNumber,
    AccTaxTypeID,
    AccTaxClassificationID
}
#[derive(Iden)]
pub enum DentistClinic {
    Table,
    Id,
    DentistId,
    ClinicId,
    Position,
    Schedule
}


