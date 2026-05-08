
#![allow(dead_code)]
pub mod boiler;
pub mod login;
mod structs;
mod request_parts;
mod middlewares;
mod helpers;
mod api;
mod app_config;
mod reports;

pub use api::dental_services::{get_dental_services, patch_dental_service, post_dental_service};
pub use api::dental_service_type::get_dental_service_types;
pub use api::clinic_capabilities::{get_clinic_capabilities, patch_clinic_capability, post_clinic_capability};
pub use api::users::{get_users, patch_user, post_user};
pub use api::roles::{create_role, get_roles, patch_role};
pub use api::role_permission::get_role_permissions;
pub use api::data_objects::get_data_objects;
pub use structs::{AppError, AuthUser, Claims, JwtConfig, ListQuery, PageResponse};

pub use login::{LoginRequest, LoginResponse};

pub use middlewares::{inject_jwt_config, require_jwt};
pub use boiler::WhoAmIResponse;
pub use api::hmo::{get_companies_for_hmo_id, get_hmo_by_id, get_hmos, patch_hmo, post_hmo};
pub use api::dentist_contracts::{get_all_dentist_contracts, get_dentist_contract,
                                 patch_dentist_contract, patch_dentist_contract_rates,
                                 post_dentist_contract};
pub use api::region::{get_region_by_id, get_regions, patch_region, post_region};
pub use api::province::{get_cities_by_province, get_provinces};
pub use api::city::get_cities;
pub use api::dental_clinic::{create_dental_clinic, get_dental_clinic_by_id, get_dental_clinics, patch_dental_clinic};
pub use api::clinic_capabilities_list::{add_clinic_capability_to_clinic,
                                        get_clinic_capabilities_for_clinic,
                                        remove_clinic_capability_from_clinic,
                                        set_clinic_capabilities_for_clinic};
pub use api::dentist::{create_dentist, get_all_dentists, get_dentist_from_id, get_dentist_names, patch_dentist};
pub use api::dentist_clinic::{add_dentist_clinic, get_all_dentist_clinics, get_clinics_for_dentist_id,
                              get_dentists_for_clinic_id, remove_dentist_clinic};
pub use api::dentist_history::get_all_dentist_histories;
pub use api::dentist_status::get_all_dentist_status;
pub use api::tax_classification::get_all_tax_classifications;
pub use api::tax_type::get_all_tax_types;
pub use api::dentist_hmo_relations::{add_except_for_hmo,
                                     add_exclusive_to_hmo,
                                     get_exclusive_to_hmos_from_dentist_id,
                                     get_not_hmos_from_dentist_id,
                                     remove_except_for_hmo,
                                     remove_exclusive_to_hmo,
};
pub use api::dentist_company_relations::{add_except_for_company,
                                         add_exclusive_to_company,
                                         get_exclusive_to_companies_from_dentist_id,
                                         get_not_companies_from_dentist_id,
                                         remove_except_for_company,
                                         remove_exclusive_to_company};

pub use api::data_files::{get_contract_file_for_dentist_id, save_contract_file_for_dentist_id};
pub use api::account_type::get_all_account_types;
pub use api::dentist_clinic_position::get_dentist_clinic_positions;
pub use api::extended_dental_clinic::get_all_clinics_and_capabilities;

pub use api::endorsement_type::get_endorsement_types;
pub use api::endorsement_billing_period_type::get_endorsement_billing_period_types;
pub use api::endorsements::{create_endorsement, get_all_endorsements, get_endorsement_by_id, patch_endorsement};
pub use api::endorsement_company::{get_endorsement_companies, post_endorsement_company};

pub use api::endorsement_rates::{get_all_endorsement_rates, patch_endorsement_rate, post_endorsement_rate, put_endorsement_rate};
pub use api::endorsement_counts::{get_all_endorsement_counts, patch_endorsement_count, post_endorsement_count, put_endorsement_count};

pub use api::endorsement_master_list_upload::upload_endorsement_master_list;
pub use api::endorsement_master_list_meta_data::get_master_list_meta_data_for_endorsement_id;
pub use api::endorsement_master_list_delete::delete_master_lists_for_endorsement_id;
pub  use api::endorsement_master_list_member::set_master_list_member_active;
pub use api::hmo_endorsement::get_endorsements_for_hmo_id;
pub use api::verification::{cancel_verification, create_verification,
                            get_all_verifications,
                            get_approval_code_for_verification_id};


pub use api::endorsement_billing_rules::{delete_billing_rule, get_billing_rules_for_endorsement_id, patch_billing_rule, post_billing_rule};

pub use api::dentist_relations::get_endorsements_for_dentist_id_handler;

/*
 master_list_member::get_master_list_members_for_endorsement and
 master_list_member::get_master_lists_with_members_for_endorsement
 both return the list of master list members for the endorsement_id. However,
 the former returns the list of master list members as a flat list, while the latter
 returns the list of uploaded master lists each with its own members.
 */
pub use api::master_list_member::{get_master_list_members_for_endorsement, get_master_lists_with_members_for_endorsement};
pub use api::master_list_member_counts::{get_service_counts_for_endorsement_id, get_service_counts_for_member_id, get_used_service_counts_for_member_id};
pub use api::endorsement_master_list_members_post_patch::{create_master_list_member, patch_master_list_member};
pub use api::verification_tooth_specifics::{get_tooth_service_types, get_tooth_surfaces};
pub use api::high_end_verification_uploading_and_approval::{download_high_end_file, list_uploaded_high_end_files, upload_high_end_file};
pub use api::high_end_verification_dentist_approval::{get_high_end_verifications, post_high_end_verification_approval};


pub use api::acc_reconciliation::{create_acc_reconciliation, get_acc_recons, get_done_verifications, reconcile_verification};
pub use api::endorsement_company_master_list_members::{get_all_member_names_from_company, save_member_name_for_company};
pub use api::verification::check_approval_code;

pub use api::hmo_utilization::{download_utilization_report, get_utilization_report};
pub use api::hmo_billing::{ get_generated_hmo_billing_reports, download_generated_report};

pub use api::test_reports::{test_generate_hmo_billing_reports};