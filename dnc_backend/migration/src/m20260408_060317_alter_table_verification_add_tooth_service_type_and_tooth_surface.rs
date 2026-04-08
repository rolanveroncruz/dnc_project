use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_table_tooth_service_type(manager).await?;
        Self::seed_tooth_service_type(manager).await?;
        Self::alter_table_verification_add_tooth_service_type(manager).await?;
        Self::create_table_tooth_surface(manager).await?;
        Self::seed_tooth_surface(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_table_tooth_service_type(manager).await?;
        Self::drop_table_tooth_surface(manager).await?;
        Ok(())
    }
}

impl Migration {
    // region: Tooth Surface
    pub async fn create_table_tooth_surface(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ToothSurface::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(ToothSurface::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(ToothSurface::Name)
                        .string()
                        .not_null()
                    )
                    .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_table_tooth_surface(manager:&SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ToothSurface::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_tooth_surface(manager:&SchemaManager<'_>, name:&str)->Result<(),DbErr> {
        let insert = Query::insert()
            .into_table(ToothServiceType::Table)
            .columns(vec![ToothServiceType::Name])
            .values_panic([Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())

    }
    pub async fn seed_tooth_surface(manager:&SchemaManager<'_>)->Result<(),DbErr> {
        Self::insert_tooth_surface(manager, "Distal").await?;
        Self::insert_tooth_surface(manager, "Facial").await?;
        Self::insert_tooth_surface(manager, "Incisal").await?;
        Self::insert_tooth_surface(manager, "Lingual").await?;
        Self::insert_tooth_surface(manager, "Mesial").await?;
        Self::insert_tooth_surface(manager, "Distal").await?;
        Self::insert_tooth_surface(manager, "Occlusal").await?;
        Ok(())
    }
    // endregion: Tooth Surface

    // region: Tooth Service Type
    pub async fn create_table_tooth_service_type(manager:&SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ToothServiceType::Table)
                        .if_not_exists()
                    .col(ColumnDef::new(ToothServiceType::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                    )
                    .col(ColumnDef::new(ToothServiceType::Name)
                        .string()
                        .not_null()
                    )
            .to_owned()
            ).await?;
        Ok(())
    }
    pub async fn drop_table_tooth_service_type(manager:&SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ToothServiceType::Table).to_owned()).await?;
        Ok(())
    }
    pub async fn insert_tooth_service_type(manager:&SchemaManager<'_>, name: &str) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(ToothServiceType::Table)
            .columns(vec![ToothServiceType::Name])
            .values_panic([Expr::val(name)])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }
    pub async fn seed_tooth_service_type(manager:&SchemaManager<'_>) -> Result<(), DbErr> {
        Self::insert_tooth_service_type(manager, "First Time").await?;
        Self::insert_tooth_service_type(manager, "Root Cleaning").await?;
        Self::insert_tooth_service_type(manager, "Retreatment").await?;
        Ok(())
    }

    // endregion: Tooth Service Type

    // region Alter table verification
    pub async fn alter_table_verification_add_tooth_service_type(manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_column( ColumnDef::new(Verification::ToothServiceTypeId)
                        .integer()
                    )
                    .add_foreign_key(TableForeignKey::new()
                        .name("fk_verification_table_tooth_service_type_id")
                        .from_tbl(Verification::Table)
                        .from_col(Verification::ToothServiceTypeId)
                        .to_tbl(ToothServiceType::Table)
                        .to_col(ToothServiceType::Id)
                    ).to_owned(),
            ).await?;
        Ok(())

    }

    // endregion Alter table verification
}
#[derive(Iden)]
enum ToothServiceType {
    Table,
    Id,
    Name
}

#[derive(Iden)]
enum ToothSurface{
    Table,
    Id,
    Name
}
#[derive(Iden)]
enum Verification {
    Table,
    ToothServiceTypeId,
}
