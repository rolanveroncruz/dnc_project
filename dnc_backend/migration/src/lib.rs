pub use sea_orm_migration::prelude::*;

mod m20251205_063628_create_table_dataobject;
mod m20251205_075427_create_table_permission;
mod m20251205_075435_create_table_role;
mod m20251205_075445_create_table_role_permission;
mod m20251205_075454_create_table_user;
mod m20251221_124454_create_table_dental_services;
mod m20251221_124505_create_table_clinic_capabilities;
mod m20260108_051749_create_table_hmo;
mod m20260119_112338_create_table_dentist_contract;
mod m20260119_112532_create_table_dentist_contract_service_rates;
mod m20260119_131647_add_dentist_contract_permission_and_role_permissions;
mod m20260126_063012_create_tables_dental_clinic;

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
            Box::new(m20251221_124454_create_table_dental_services::Migration),
            Box::new(m20251221_124505_create_table_clinic_capabilities::Migration),
            Box::new(m20260108_051749_create_table_hmo::Migration),
            Box::new(m20260119_112338_create_table_dentist_contract::Migration),
            Box::new(m20260119_112532_create_table_dentist_contract_service_rates::Migration),
            Box::new(m20260119_131647_add_dentist_contract_permission_and_role_permissions::Migration),
            Box::new(m20260126_063012_create_tables_dental_clinic::Migration),
        ]
    }
}
