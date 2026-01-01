use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251205_075435_create_table_role::Role;

// password hashing libraries
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(ColumnDef::new(User::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Email)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Password)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::RoleId)
                        .integer()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Active)
                    .boolean()
                    .default(true)
                    .not_null()
                    )
                    .col(ColumnDef::new(User::LastModifiedBy)
                        .string()
                        .not_null()
                        .default("system")
                    )
                    .col(ColumnDef::new(User::LastModifiedOn)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_role_id_foreign_key")
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    ).to_owned()
            ).await?;

        // 2. Create the Admin user
        Self::add_user(manager, "admin", "admin@dnc.com.ph", "password", "Administrator").await?;
        Self::add_user(manager, "noperms", "noperms@dnc.com.ph", "noperms", "NoPerms").await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}
impl Migration {

    async fn add_user(manager: &SchemaManager<'_>, name: &str, email: &str, password: &str, role_name: &str)->Result<(), DbErr>{
        let mut rng = OsRng;
        let salt = SaltString::generate(&mut rng);
        let argon2 = Argon2::default();
        let password_hash= argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();


        let insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Name, User::Email, User::Password, User::RoleId])
            .select_from(
                Query::select()
                    .expr(Expr::val(name))
                    .expr(Expr::val(email))
                    .expr(Expr::val(password_hash))

                    .expr(
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(
                                Query::select()
                                    .column(Role::Id)
                                    .from(Role::Table)
                                    .and_where(Expr::col(Role::Name).eq(role_name))
                                    .limit(1)
                                    .to_owned()
                                    .into()
                            )
                        )
                    )
                    .to_owned()
            ).map_err(|e|DbErr::Custom(format!("{:?}", e)))?
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum User{
    Table,
    Id,
    Name,
    Email,
    Password,
    RoleId,
    Active,
    LastModifiedBy,
    LastModifiedOn,
}
