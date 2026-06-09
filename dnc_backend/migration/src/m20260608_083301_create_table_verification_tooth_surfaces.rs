use sea_orm_migration::{prelude::* };
use crate::m20260408_060317_alter_table_verification_add_tooth_service_type_and_tooth_surface::ToothSurface;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_verification_tooth_surfaces(manager).await?;
        Self::copy_existing_tooth_surface_ids(manager).await?;
        Self::drop_verification_tooth_surface_id(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::add_verification_tooth_surface_id(manager).await?;
        Self::restore_verification_tooth_surface_id(manager).await?;
        Self::create_verification_tooth_surface_id_index(manager).await?;
        Self::drop_verification_tooth_surfaces(manager).await?;
        Ok(())
    }
}

impl Migration{
    async fn create_verification_tooth_surfaces(manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager.create_table(
            Table::create()
                .table(VerificationToothSurfaces::Table)
                .if_not_exists()
                .col( ColumnDef::new(VerificationToothSurfaces::Id)
                    .integer()
                    .primary_key()
                    .auto_increment()
                    .not_null()
                )
                .col( ColumnDef::new(VerificationToothSurfaces::VerificationId)
                    .integer()
                    .not_null()
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_verification_tooth_surfaces_verification_id")
                        .from(VerificationToothSurfaces::Table, VerificationToothSurfaces::VerificationId)
                        .to(Verification::Table, Verification::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .col( ColumnDef::new(VerificationToothSurfaces::ToothSurfaceId)
                    .integer()
                    .not_null()
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_verification_tooth_surfaces_tooth_surface_id")
                        .from(VerificationToothSurfaces::Table, VerificationToothSurfaces::ToothSurfaceId)
                        .to(ToothSurface::Table, ToothSurface::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                )
                .to_owned()
        ).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_verification_tooth_surfaces_verification_id")
                    .table(VerificationToothSurfaces::Table)
                    .col(VerificationToothSurfaces::VerificationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_verification_tooth_surfaces_tooth_surface_id")
                    .table(VerificationToothSurfaces::Table)
                    .col(VerificationToothSurfaces::ToothSurfaceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_verification_tooth_surfaces_unique")
                    .table(VerificationToothSurfaces::Table)
                    .col(VerificationToothSurfaces::VerificationId)
                    .col(VerificationToothSurfaces::ToothSurfaceId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
    async fn copy_existing_tooth_surface_ids(manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                 INSERT INTO verification_tooth_surfaces
                (verification_id, tooth_surface_id)
                SELECT
                    id,
                    tooth_surface_id
                FROM verification
                WHERE tooth_surface_id IS NOT NULL;
                "#
            ).await?;
        Ok(())
    }
    async fn drop_verification_tooth_surface_id(manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .drop_column(Verification::ToothSurfaceId)
                    .to_owned()
            ).await?;
        Ok(())

    }


    async fn add_verification_tooth_surface_id(manager: &SchemaManager<'_>)-> Result<(), DbErr>{
        manager
            .alter_table(
                Table::alter()
                    .table(Verification::Table)
                    .add_column(
                        ColumnDef::new(Verification::ToothSurfaceId)
                            .integer()
                            .null(),
                    )
                   .add_foreign_key(
                        TableForeignKey::new()
                        .name("fk_verification_tooth_surface_id")
                        .from_tbl(Verification::Table)
                        .from_col(Verification::ToothSurfaceId)
                        .to_tbl(ToothSurface::Table)
                        .to_col(ToothSurface::Id)
                        .on_delete(ForeignKeyAction::Restrict),
                   )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
    async fn restore_verification_tooth_surface_id(
        manager: &SchemaManager<'_>,
    ) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
            UPDATE verification v
            SET tooth_surface_id = x.tooth_surface_id
            FROM (
                SELECT DISTINCT ON (verification_id)
                    verification_id,
                    tooth_surface_id
                FROM verification_tooth_surfaces
                ORDER BY verification_id, tooth_surface_id
            ) x
            WHERE v.id = x.verification_id;
            "#,
            )
            .await?;

        Ok(())
    }

    async fn create_verification_tooth_surface_id_index(
        manager: &SchemaManager<'_>,
    ) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_verification_tooth_surface_id")
                    .table(Verification::Table)
                    .col(Verification::ToothSurfaceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn drop_verification_tooth_surfaces(manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager.drop_table(
            Table::drop()
                .table(VerificationToothSurfaces::Table)
                .if_exists()
                .to_owned()).await?;
        Ok(())
    }
}
/*
 VerificationToothSurfaces lists all tooth surfaces that verifications' dental service is performed on.
 */
#[derive(DeriveIden)]
pub enum VerificationToothSurfaces{
    Table,
    Id,
    VerificationId,
    ToothSurfaceId,
}
#[derive(DeriveIden)]
pub enum Verification {
    Id,
    Table,
    ToothSurfaceId,
}
