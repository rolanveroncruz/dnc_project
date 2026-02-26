// m20260126_063012_create_tables_dental_clinic.rs
use sea_orm_migration::prelude::*;

use crate::m20251221_124505_create_table_clinic_capabilities::ClinicCapability;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_table_region(manager).await?;
        Self::create_table_province(manager).await?;
        Self::create_table_city(manager).await?;
        Self::create_tax_type_table(manager).await?;
        Self::insert_tax_type_seed(manager).await?;
        Self::create_account_type_table(manager).await?;
        Self::insert_account_type_seed(manager).await?;
        Self::create_tax_classification_table(manager).await?;
        Self::insert_tax_classification_seed(manager).await?;
        Self::create_table_dental_clinic(manager).await?;
        Self::create_table_clinic_capabilities_list(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ClinicCapabilitiesList::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(DentalClinic::Table).to_owned()).await?;
        Self::drop_account_type_table(manager).await?;
        Self::drop_tax_classification_table(manager).await?;
        Self::drop_tax_type_table(manager).await?;
        manager.drop_table(Table::drop().table(City::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Province::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Region::Table).to_owned()).await?;

        Ok(())
    }
}
impl Migration {

    async fn create_table_region(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table region");
        manager
            .create_table(
                Table::create()
                    .table(Region::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Region::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Region::Name)
                        .string()
                        .not_null()
                    )
                    .index(Index::create()
                        .name("region_name_index")
                        .table(Region::Table)
                        .col(Region::Name)
                        .unique()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_table_province(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table province");
        manager
            .create_table(
                Table::create()
                    .table(Province::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Province::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Province::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(Province::RegionId)
                        .integer()
                        .not_null()
                    )
                    .index(Index::create()
                        .name("province_name_region_id_index")
                        .table(Province::Table)
                        .col(Province::Name)
                        .col(Province::RegionId)
                        .unique()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("state_region_id_foreign_key")
                        .from(Province::Table, Province::RegionId)
                        .to(Region::Table, Region::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_table_city(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table city");
        manager
            .create_table(
                Table::create()
                    .table(City::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(City::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(City::Name).string().not_null()
                    )
                    .col(ColumnDef::new(City::ProvinceId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("city_province_id_foreign_key")
                        .from(City::Table, City::ProvinceId)
                        .to(Province::Table, Province::Id)
                    )
                    .index(Index::create()
                        .name("city_name_province_id_index")
                        .table(City::Table)
                        .col(City::Name)
                        .col(City::ProvinceId)
                        .unique()
                    )
                    .to_owned()
            ).await
    }

    async fn create_table_dental_clinic(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table dental_clinic");
        manager
            .create_table(
                Table::create()
                    .table(DentalClinic::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DentalClinic::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(DentalClinic::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentalClinic::OwnerName)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::Address)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(DentalClinic::CityId)
                    .integer()
                    )
                    .col(ColumnDef::new(DentalClinic::ZipCode)
                        .string()
                    )
                    .index(Index::create()
                        .name("dental_clinic_name_address_city_zipcode_index")
                        .table(DentalClinic::Table)
                        .col(DentalClinic::Name)
                        .col(DentalClinic::Address)
                        .col(DentalClinic::CityId)
                        .col(DentalClinic::ZipCode)
                        .unique()
                    ).to_owned()
                    .col(ColumnDef::new(DentalClinic::Remarks)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::ContactNumbers)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::Email)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::Schedule)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctTIN)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctBankName)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctAccountTypeId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dental_clinic_acct_account_type_foreign_key")
                        .from(DentalClinic::Table, DentalClinic::AcctAccountTypeId)
                        .to(AccountType::Table, AccountType::Id)
                    )
                    .col(ColumnDef::new(DentalClinic::AcctAccountName)
                    .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctAccountNumber)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctTaxTypeId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dental_clinic_acct_tax_type_foreign_key")
                        .from(DentalClinic::Table, DentalClinic::AcctTaxTypeId)
                        .to(TaxType::Table, TaxType::Id)
                    )
                    .col(ColumnDef::new(DentalClinic::AcctTaxClassificationId)
                        .integer()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dental_clinic_acct_tax_classification_foreign_key")
                        .from(DentalClinic::Table, DentalClinic::AcctTaxClassificationId)
                        .to(TaxClassification::Table, TaxClassification::Id)
                    )
                    .col(ColumnDef::new(DentalClinic::AcctTradeName)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::AcctTaxpayerName)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::Active)
                        .boolean()
                        .default(true)
                    )
                    .col(ColumnDef::new(DentalClinic::LastModifiedBy)
                        .string()
                        .not_null()
                        .default("system")
                    )
                    .col(ColumnDef::new(DentalClinic::LastModifiedOn)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .foreign_key(ForeignKey::create()
                        .name("dental_clinic_city_id_foreign_key")
                        .from(DentalClinic::Table, DentalClinic::CityId)
                        .to(City::Table, City::Id)
                    )
                    .to_owned()
            ).await
    }

    async fn create_table_clinic_capabilities_list(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table clinic_capabilities_list");
        manager
            .create_table(
                Table::create().table(ClinicCapabilitiesList::Table)
                .if_not_exists()
                .col(ColumnDef::new(ClinicCapabilitiesList::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key()
                )
                .col(ColumnDef::new(ClinicCapabilitiesList::ClinicId)
                    .integer()
                    .not_null()
                )
                .col(ColumnDef::new(ClinicCapabilitiesList::CapabilityId)
                    .integer()
                    .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .name("clinic_capabilities_list_clinic_id_foreign_key")
                    .from(ClinicCapabilitiesList::Table, ClinicCapabilitiesList::ClinicId)
                    .to(DentalClinic::Table, DentalClinic::Id)
                )
               .foreign_key(ForeignKey::create()
                   .name("clinic_capabilities_list_capability_id_foreign_key")
                   .from(ClinicCapabilitiesList::Table, ClinicCapabilitiesList::CapabilityId)
                   .to(ClinicCapability::Table, ClinicCapability::Id)
               )
                .to_owned()

        ).await
    }


    pub async fn create_tax_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table tax_type");
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
        tracing::info!("Inserting tax_type seed");
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
        tracing::info!("Creating table tax_classification");
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
        tracing::info!("Dropping table tax_classification");
        manager.drop_table(Table::drop().table(TaxClassification::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_tax_classification_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Inserting tax_classification seed");
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

    pub async fn create_account_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Creating table account_type");
        manager
            .create_table(
                Table::create()
                    .table(AccountType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AccountType::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(AccountType::Name)
                        .string()
                        .not_null()
                    ).to_owned()
            ).await?;

        Ok(())
    }
    pub async fn drop_account_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Dropping table account_type");
        manager.drop_table(Table::drop().table(AccountType::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_account_type_seed(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        tracing::info!("Inserting account_type seed");
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(AccountType::Table)
                    .columns([AccountType::Name])
                    .values_panic(["Savings".into()])
                    .values_panic(["Current / Checking".into()])
                    .to_owned(),
            ).await?;
        Ok(())
    }

}


#[derive(Iden)]
pub enum Region {
    Table,
    Id,
    Name,
}


#[derive(Iden)]
pub enum Province{
    Table,
    Id,
    Name,
    RegionId,
}


#[derive(Iden)]
pub enum City {
    Table,
    Id,
    Name,
    ProvinceId,
}
#[derive(Iden)]
pub enum ClinicCapabilitiesList {
    Table,
    Id,
    ClinicId,
    CapabilityId,
}

#[derive(Iden)]
pub enum AccountType{
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
pub enum TaxType{
    Table,
    Id,
    Name,
}
#[derive(Iden)]
pub enum DentalClinic {
    Table,
    Id,
    Name,
    OwnerName,
    Address,
    CityId,
    ZipCode,
    Schedule,
    ContactNumbers,
    Email,
    Remarks,
    AcctTIN,
    AcctBankName,
    AcctAccountTypeId,
    AcctAccountName,
    AcctAccountNumber,
    AcctTaxTypeId,
    AcctTaxClassificationId,
    AcctTaxpayerName,
    AcctTradeName,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
