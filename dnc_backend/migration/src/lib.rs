pub use sea_orm_migration::prelude::*;

mod m20251205_063628_create_table_dataobject;
mod m20251205_075427_create_table_permission;
mod m20251205_075435_create_table_role;
mod m20251205_075445_create_table_role_permission;
mod m20251205_075454_create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251205_063628_create_table_dataobject::Migration),
            Box::new(m20251205_075427_create_table_permission::Migration),
            Box::new(m20251205_075435_create_table_role::Migration),
            Box::new(m20251205_075445_create_table_role_permission::Migration),
            Box::new(m20251205_075454_create_table_user::Migration),
        ]
    }
}
