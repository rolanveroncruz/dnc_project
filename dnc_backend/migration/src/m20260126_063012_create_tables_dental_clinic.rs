// m20260126_063012_create_tables_dental_clinic.rs
use sea_orm_migration::prelude::*;

use crate::m20251221_124505_create_table_clinic_capabilities::ClinicCapability;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_table_region(manager).await?;
        Self::create_table_state(manager).await?;
        Self::create_table_city(manager).await?;
        Self::create_table_dental_clinic(manager).await?;
        Self::create_table_clinic_capabilities_list(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Region::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DentalClinic::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ClinicCapabilitiesList::Table).to_owned())
            .await?;

        Ok(())
    }
}
impl Migration {



    async fn create_table_region(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Region::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }


    async fn create_table_state(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(State::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(State::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(State::Name).string().not_null())
                    .col(ColumnDef::new(State::RegionId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("state_region_id_foreign_key")
                        .from(State::Table, State::RegionId)
                        .to(Region::Table, Region::Id)
                    )
                    .to_owned(),
            )
            .await
    }


    async fn create_table_city(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(City::StateId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("city_state_id_foreign_key")
                        .from(City::Table, City::StateId)
                        .to(State::Table, State::Id)
                    )
                    .to_owned()
            ).await
    }


    async fn create_table_dental_clinic(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(DentalClinic::Remarks)
                        .string()
                    )
                    .col(ColumnDef::new(DentalClinic::ContactNumbers)
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
}


#[derive(Iden)]
pub enum Region {
    Table,
    Id,
    Name,
}


#[derive(Iden)]
pub enum State {
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
    StateId,
}
#[derive(Iden)]
pub enum ClinicCapabilitiesList {
    Table,
    Id,
    ClinicId,
    CapabilityId,
}

#[derive(Iden)]
pub enum DentalClinic {
    Table,
    Id,
    Name,
    Address,
    CityId,
    ZipCode,
    Remarks,
    ContactNumbers,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
