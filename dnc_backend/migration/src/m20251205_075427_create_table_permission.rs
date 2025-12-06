use sea_orm_migration::{prelude::* };
use sea_orm_migration::sea_query::extension::postgres::Type;
use crate::m20251205_063628_create_table_dataobject::DataObject;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // 1. Create Type in Postgres
        manager
            .create_type(
                Type::create()
                    .as_enum(PermissionAction::EnumName)
                    .values([
                        PermissionAction::Create,
                        PermissionAction::Read,
                        PermissionAction::Update
                    ]).to_owned(),
            )
            .await?;


        // 2. Create Table
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Permission::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Permission::DataObjectId)
                        .integer()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("permission_data_object_id_foreign_key")
                            .from(Permission::Table, Permission::DataObjectId)
                            .to(DataObject::Table, DataObject::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(ColumnDef::new(Permission::Action)
                        .custom(PermissionAction::EnumName)
                        .not_null()
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create Initial Data
        let insert = Query::insert()
            .into_table(Permission::Table)
            .columns([Permission::DataObjectId, Permission::Action])
            .select_from(
                Query::select()
                    .column(DataObject::Id)
                    .expr(Expr::val("create").cast_as(PermissionAction::EnumName))
                    .from(DataObject::Table)
                    .and_where(Expr::col(DataObject::Name).eq("User"))
                    .to_owned()
            )
            .map_err(|e|DbErr::Custom(format!("{:?}", e)))?
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PermissionAction{
    #[sea_orm(iden="permission_action_enum")]
    EnumName,
    #[sea_orm(iden="create")]
    Create,
    #[sea_orm(iden="read")]
    Read,
    #[sea_orm(iden="update")]
    Update,
}

#[derive(Iden)]
pub enum Permission{
    Table,
    Id,
    DataObjectId,
    Action,
}
