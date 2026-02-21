use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_endorsement_company_table(manager).await?;
        Self::create_endorsement_type_table(manager).await?;
        Self::seed_endorsement_type_table(manager, "Retainer Only").await?;
        Self::seed_endorsement_type_table(manager, "Retainer With Special Services").await?;
        Self::seed_endorsement_type_table(manager, "Retainer And Fee Per Service").await?;
        Self::create_endorsement_billing_period_type_table(manager).await?;
        Self::seed_endorsement_billing_period_type_table(manager, "Billed Annually").await?;
        Self::seed_endorsement_billing_period_type_table(manager, "Billed Monthly").await?;
        Self::create_endorsements_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_endorsement_company_table(manager).await?;
        Self::drop_endorsement_type_table(manager).await?;
        Self::drop_endorsement_billing_period_type_table(manager).await?;
        Self::drop_endorsements_table(manager).await?;
        Ok(())
    }
}
impl Migration {
    /*
    Endorsement Companies are the list of companies that are endorsed by HMOs.
     */
    pub async fn create_endorsement_company_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndorsementCompany::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(EndorsementCompany::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .col(ColumnDef::new(EndorsementCompany::Name)
                        .string()
                        .not_null()
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_endorsement_company_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(EndorsementCompany::Table).to_owned()).await?;
        Ok(())
    }

    /*
    Endorsement Types are "Retainer Only", "Retainer With Special Services", and "Retainer And Fee Per Service".
     */
    pub async fn create_endorsement_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndorsementType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EndorsementType::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .col(ColumnDef::new(EndorsementType::Name).string().not_null())
                    .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn seed_endorsement_type_table(manager: &SchemaManager<'_>, name: &str) -> Result<(), DbErr> {
        let insert = Query::insert()
                .into_table(EndorsementType::Table)
                .columns([EndorsementType::Name])
                .values_panic([name.into()])
            .to_owned();
        manager.exec_stmt(insert).await
    }
    pub async fn drop_endorsement_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(EndorsementType::Table).to_owned()).await?;
        Ok(())
    }

    /*
    Billing Period Types are "Billed Annually" and "Billed Monthly".
     */
    pub async fn create_endorsement_billing_period_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndorsementBillingPeriodType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EndorsementBillingPeriodType::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(EndorsementBillingPeriodType::Name)
                        .string()
                        .not_null()
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn seed_endorsement_billing_period_type_table(manager: &SchemaManager<'_>, name: &str) -> Result<(), DbErr> {
        let insert = Query::insert()
                .into_table(EndorsementBillingPeriodType::Table)
                .columns([EndorsementBillingPeriodType::Name])
                .values_panic([name.into()])
            .to_owned();
        manager.exec_stmt(insert).await
    }
    pub async fn drop_endorsement_billing_period_type_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(EndorsementBillingPeriodType::Table).to_owned()).await?;
        Ok(())
    }

    /*
    Endorsements are the authorizations HMOs give to treat a list of members.
     */
    pub async fn create_endorsements_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Endorsement::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(Endorsement::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .to_owned()
            ).await?;
       Ok(())
    }
    pub async fn drop_endorsements_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Endorsement::Table).to_owned()).await?;
        Ok(())
    }

}
#[derive(Iden)]
pub enum EndorsementCompany{
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum EndorsementType{
    Table,
    Id,
    Name,
}
#[derive(Iden)]
pub enum EndorsementBillingPeriodType{
    Table,
    Id,
    Name,
}


#[derive(Iden)]
pub enum Endorsement {
    Table,
    Id,
    EndorsementCompanyId, //
    EndorsementTypeId,    //
    AgreementCorpNumber,
    DateStart,
    DateEnd,
    EndorsementBillingPeriodTypeId,
    RetainerFee,
    Remarks,
    EndorsementMethod,
    MemberCount,
    LastBilledDate
}





