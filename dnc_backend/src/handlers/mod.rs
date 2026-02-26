
pub mod boiler;
pub mod login;
mod structs;
mod request_parts;
mod middlewares;
mod helpers;
mod api;

pub use api::dental_services::{get_dental_services, post_dental_service, patch_dental_service};
pub use api::dental_service_type::get_dental_service_types;
pub use api::clinic_capabilities::{get_clinic_capabilities, post_clinic_capability, patch_clinic_capability};
pub use api::users::{get_users, post_user, patch_user};
pub use api::roles::{get_roles, create_role,patch_role};
pub use api::role_permission::get_role_permissions;
pub use api::data_objects::get_data_objects;
pub use structs::{AuthUser, Claims, JwtConfig, ListQuery, PageResponse, AppError};

pub use login::{LoginRequest, LoginResponse};

pub use middlewares::{inject_jwt_config, require_jwt};
pub use boiler::WhoAmIResponse;
pub use api::hmo::{get_hmos, get_hmo_by_id, post_hmo, patch_hmo};
pub use api::dentist_contracts::{get_all_dentist_contracts, get_dentist_contract,
                                 post_dentist_contract, patch_dentist_contract,
                                 patch_dentist_contract_rates};
pub use api::region::{get_regions, get_region_by_id, post_region, patch_region};
pub use api::province::{get_provinces, get_cities_by_province};
pub use api::city::{get_cities};
pub use api::dental_clinic::{get_dental_clinics, get_dental_clinic_by_id, create_dental_clinic, patch_dental_clinic};
pub use api::clinic_capabilities_list::{get_clinic_capabilities_for_clinic,
                                        add_clinic_capability_to_clinic,
                                        remove_clinic_capability_from_clinic,
                                        set_clinic_capabilities_for_clinic};
pub use api::dentist::{get_all_dentists, get_dentist_from_id, create_dentist, patch_dentist};
pub use api::dentist_clinic::{get_all_dentist_clinics, get_clinics_for_dentist_id, get_dentists_for_clinic_id,
                              add_dentist_clinic, remove_dentist_clinic};
pub use api::dentist_history::{get_all_dentist_histories};
pub use api::dentist_status::{get_all_dentist_status};
pub use api::tax_classification::get_all_tax_classifications;
pub use api::tax_type::get_all_tax_types;
pub use api::dentist_hmo_relations::{get_exclusive_to_hmos_from_dentist_id, 
                                     add_exclusive_to_hmo,
                                     remove_exclusive_to_hmo,
                                     get_not_hmos_from_dentist_id,
                                     add_except_for_hmo,
                                     remove_except_for_hmo,
};

pub use api::data_files::{save_contract_file_for_dentist_id, get_contract_file_for_dentist_id};
pub use api::account_type::get_all_account_types;
pub use api::dentist_clinic_position::get_dentist_clinic_positions;
pub use api::extended_dental_clinic::get_all_clinics_and_capabilities;

pub use api::endorsement_type::get_endorsement_types;
pub use api::endorsement_billing_period_type::get_endorsement_billing_period_types;