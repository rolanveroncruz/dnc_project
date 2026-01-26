
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
pub use api::city::get_cities;
pub use api::state::get_states;
pub use api::region::get_regions;
pub use api::dental_clinic::{get_dental_clinics, get_dental_clinic_by_id, post_dental_clinic, patch_dental_clinic};