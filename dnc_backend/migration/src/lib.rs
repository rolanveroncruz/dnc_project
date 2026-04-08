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
mod m20260126_151643_add_dental_clinic_permission_and_role_permissions;
mod m20260126_161604_create_table_dentists;
mod m20260129_023740_insert_dentist_and_dental_clinics_from_xlsx;
mod m20260205_013824_create_dentist_hmo_relations;
mod m20260220_082933_create_endorsement_tables;
mod m20260220_122937_add_endorsement_permission_and_role_permissions;
mod m20260307_083354_add_endorsement_rates_masterlists;
mod m20260309_060904_add_endorsement_rates_mls_permissions_and_role_permissions;
mod m20260319_052702_add_verification_tables;
mod m20260320_142702_add_verification_permissions_and_role_permissions;
mod m20260321_115732_add_dentist_company_relations;
mod m20260323_150430_endorsement_billing_rule_table;
mod m20260326_172948_alter_table_verifications_fk_status_id;
mod m20260331_062346_update_dental_services_that_need_tooth_for_verification;
mod m20260331_160341_alter_table_verification_add_tooth_id_column;
mod m20260408_060317_alter_table_verification_add_tooth_service_type_and_tooth_surface;

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
            Box::new(m20260126_151643_add_dental_clinic_permission_and_role_permissions::Migration),
            Box::new(m20260126_161604_create_table_dentists::Migration),
            Box::new(m20260129_023740_insert_dentist_and_dental_clinics_from_xlsx::Migration),
            Box::new(m20260205_013824_create_dentist_hmo_relations::Migration),
            Box::new(m20260220_082933_create_endorsement_tables::Migration),
            Box::new(m20260220_122937_add_endorsement_permission_and_role_permissions::Migration),
            Box::new(m20260307_083354_add_endorsement_rates_masterlists::Migration),
            Box::new(m20260309_060904_add_endorsement_rates_mls_permissions_and_role_permissions::Migration),
            Box::new(m20260319_052702_add_verification_tables::Migration),
            Box::new(m20260320_142702_add_verification_permissions_and_role_permissions::Migration),
            Box::new(m20260321_115732_add_dentist_company_relations::Migration),
            Box::new(m20260323_150430_endorsement_billing_rule_table::Migration),
            Box::new(m20260326_172948_alter_table_verifications_fk_status_id::Migration),
            Box::new(m20260331_062346_update_dental_services_that_need_tooth_for_verification::Migration),
            Box::new(m20260331_160341_alter_table_verification_add_tooth_id_column::Migration),
            Box::new(m20260408_060317_alter_table_verification_add_tooth_service_type_and_tooth_surface::Migration),
        ]
    }
}
